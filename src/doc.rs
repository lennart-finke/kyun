pub use crate::flie_type::FileType;
use crate::row::{self, SearchDirection};
pub use crate::row::SingleRow;

use crate::Position;
use std::fs::{self, File};
use std::io::{Error,Write};


// ------------------------
// 文件结构体
// ------------------------
#[derive(Default)]
pub struct Doc{
    pub file_name: Option<String>, // 文件名
    file_type: FileType, // 文件类型
    modified: bool, // 是否修改过
    rows: Vec<SingleRow>, // 文件每一行
    count: usize, // 文件总共有多少行
}

impl Doc {
    // 打开一个文件，且用该文件的内容初始化并返回一个Doc
    pub fn open_file(file_name: &str) -> Result<Self,std::io::Error>{
        // 读取文件内容
        let file_content = fs::read_to_string(file_name)?;
        // 读取文件类型
        let file_type = FileType::from(file_name);

        // 将文件内容按行分割，每行存储在一个SingleRow中
        let mut rows = Vec::new();
        for line in file_content.lines(){
            rows.push(SingleRow::from(line));
        }

        Ok(Self{
                count: rows.len(),
                rows,
                file_name: Some(file_name.to_string()),
                modified: false,
                file_type,
        })
    }

    // 根据字符串创建Doc
    pub fn content_from_string(contents: String) ->Result<Self,std::io::Error>{
        let mut rows = Vec::new();

        for line in contents.lines(){
            rows.push(SingleRow::from(line));
        }

        Ok(Self{
            count: rows.len(),
            rows,
            file_name: None,
            modified: false,
            file_type: FileType::default(),
        })
    }

    pub fn get_file_type(&self)->String{
        return self.file_type.name();
    }

    // 获取doc指定的行
    pub fn row(&self,index: usize) -> Option<&SingleRow>{
        return  self.rows.get(index);
    }

    pub fn is_empty(&self) ->bool{
        return self.rows.is_empty();
    }

    // 获取doc行数
    pub fn get_len(&self) -> usize{
        return self.rows.len();
    }

    pub fn is_modified(&self) -> bool{
        return self.modified;
    }

    // 取消指定行之后所有行的高亮
    pub fn cancel_high_light_rows(&mut self, begin: usize) {
        // 减一使包括指定行
        let begin = begin.saturating_sub(1);
        // 取消之后每一行之后的高亮
        for row in self.rows.iter_mut().skip(begin) {
            row.is_highligthing = false;
        }
    }

    fn insert_line(&mut self, position :&Position){

        // 如果插入位置比当前文本行大，则不插入
        if position.y > self.rows.len(){
            return;
        }

        // 如果在最后一行插入行
        if position.y == self.rows.len(){
            self.rows.push(SingleRow::default());
            return;
        }

        // 在行中间插入新行
        let current_row = &mut self.rows[position.y];
        let suf_row = current_row.split_content(position.x);
        self.rows.insert(position.y+1, suf_row);
    }


    pub fn insert(&mut self,position :&Position,c: char){
        if position.y > self.rows.len(){
            return;
        }

        self.modified = true;

        // 插入字符
        if c=='\n' {
            self.insert_line(position);
        
        // 在末尾行插入字符
        }else if position.y == self.rows.len(){
            let mut row = SingleRow::default();
            row.insert_at_positon(0, c);
            self.rows.push(row);

        // 在当前行的位置插入
        }else{
            let row = &mut self.rows[position.y];
            row.insert_at_positon(position.x, c);
        }

        self.cancel_high_light_rows(position.y);
    }

    // 删除指定位置的字符
    pub fn delete_at_position(&mut self, position :&Position) {
        let length = self.rows.len();
        // 位置不对，直接返回
        if position.y >= length{
            return;
        }

        self.modified = true;

        // 某一行的最后一个字符,则将下一行前移
        if position.x == self.rows[position.y].get_content_len() && position.y + 1 < length {
            let next_row = self.rows.remove(position.y + 1);
            let row = &mut self.rows[position.y];
            row.appand_row(&next_row);
        }else{
            let row = &mut self.rows[position.y];
            row.delete_at_position(position.x);
        }

        self.cancel_high_light_rows(position.y);
    }

    // Doc保存到文件中
    pub fn save_doc(&mut self) -> Result<(),Error>{

        // 如果指定了文件名
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(&file_name);

            // 写入每一行的内容，并在结尾添加换行符号
            for row in &mut self.rows{
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            self.modified = false;
        }

        Ok(())
    }

    // -----------------------------------------
    // 从Doc中查找字符串，可以在这里添加多行一起查找
    // -----------------------------------------
    pub fn find(&self, query: &str, position: &Position, direction: SearchDirection) -> Option<Position>{
        if position.y >= self.rows.len(){
            return None;
        }

        let mut position = Position{x: position.x, y: position.y};

        // 起始行
        let mut begin_line;
        if direction == SearchDirection::Forward{
            begin_line = position.y;
        }else{
            begin_line = 0;
        }

        // 终止行
        let mut end_line;
        if direction == SearchDirection::Backward{
            end_line = self.rows.len();
        }else{
            end_line = position.y.saturating_add(1);
        }

        // 逐行查找，找到就返回
        for _ in begin_line..end_line{
         
            if let Some(row) = self.rows.get(position.y){
                if let Some(x) = row.search_query_str(&query, position.x, direction){
                    position.x = x;
                    return Some(position);
                }

                // 更新查找位置
                if direction == SearchDirection::Forward{
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                }else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].get_content_len();
                }

            }else{
                return None;
            }
        }

        None
    }

    // 高亮显示指定的行
    pub fn high_light(&mut self, word: &Option<String>, stop: Option<usize>) {
        let mut begin_comment = false;

        // 如果没有指定行数，则高亮显示所有行
        let stop = if let Some(stop) = stop{
            if stop.saturating_add(1) < self.rows.len(){
                stop.saturating_add(1)
            }else{
                self.rows.len()
            }
        }else{
            self.rows.len()
        };


        for row in &mut self.rows[..stop] {
            begin_comment = row.highlight_control(
                &self.file_type.highlighting_options(),
                word, 
                begin_comment);
        }
    }



}