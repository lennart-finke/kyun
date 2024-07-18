pub use crate::high_lighting;
pub use crate::flie_type::SearchDirection;
pub use crate::flie_type::HighlightingOptions;
use crossterm::style::SetForegroundColor;
use unicode_segmentation::UnicodeSegmentation;


pub struct SingleRow{
    row_content: String, // 该行实际保存的文本内容
    char_display_style: Vec<high_lighting::Type>, // 每个图元的显示样式
    pub is_highligthing: bool,
    content_len: usize,
    current_row: usize,
}

// 实现默认trait
impl Default for SingleRow{
    fn default() -> Self{
        SingleRow{
            row_content: String::new(),
            char_display_style: Vec::new(),
            is_highligthing: false,
            content_len: 0,
            current_row: 0
        }
    }
}

// 根据一个字符串中创建一个row实例
impl From<&str> for SingleRow{
    fn from(slice :&str) -> Self{
        Self{
            row_content: String::from(slice),
            char_display_style: Vec::new(),
            is_highligthing: false,
            content_len: slice.graphemes(true).count(),
            current_row: 0
        }
    }
}

// 判断给定字符是不是标点符号
fn check_char(c :char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}

impl SingleRow {

    pub fn get_content_len(&self) -> usize{
        self.content_len
    }

    // 根据设置的样式为字符串添加显示串，返回设置好的字符串
    pub fn draw_content(&self, begin: usize,end: usize)->String {
        // 获取有效的显示范围
        let end = std::cmp::min(self.get_content_len(), end);
        let start = std::cmp::min(begin, end);
    
        // show_string为添加显示样式后的字符串
        let mut show_string = String::new();
        let mut current_show_style = &high_lighting::Type::None;

        for (index,graheme) in self.row_content[0..].graphemes(true).enumerate().skip(start).take(end-start){
            match graheme.chars().next(){
                Some(c)=>{
                    // 获取当前字符的显示样式
                    let show_style = self.char_display_style.get(index).unwrap_or(&high_lighting::Type::None);

                    // 更改显示样式，否则之后所有字符公用一种样式
                    if show_style != current_show_style{
                        current_show_style = show_style;
                        show_string.push_str(format!("{}",SetForegroundColor(show_style.to_color())).as_str())
                    }

                    // 添加字符
                    if c == '\t'{
                        show_string.push_str(" ");
                    }else{
                        show_string.push(c);
                    }

                },
                None=>{
                }
            }
        }

        // 在末尾添加重置所有样式的命令
        show_string.push_str(format!("{}",SetForegroundColor(crossterm::style::Color::Reset)).as_str());
        return show_string;
    }

    pub fn row_is_empty(&self) -> bool{
        return  self.content_len <= 1;
    }

    // 附加新字符串
    pub fn appand_row(&mut self,appand: &Self){
        self.row_content = format!("{}{}",self.row_content,appand.row_content);
        self.content_len += appand.content_len;
    }

    // 将字符串转为ASCII码
    pub fn as_bytes(&self) -> &[u8] {
        self.row_content.as_bytes()
    }

    // 在选定位置插入字符
    pub fn insert_at_positon(&mut self, position: usize, char_inserted: char){
        // 如果插入的位置在字符串末尾，直接附加
        if position >= self.content_len {
            self.row_content.push(char_inserted);
            self.content_len += 1;
            return;
        }

        let mut new_content = String::new();
        let mut new_content_len = 0;

        //如果插入的位置不在字符串末尾
        for (Index,graheme) in self.row_content[0..].graphemes(true).enumerate(){
            // 如果当前就是要插入的位置
            if Index == position {
                new_content.push(char_inserted);
                new_content_len += 1;
            }

            new_content.push_str(graheme);
            new_content_len += 1;
        }
        
        self.content_len = new_content_len;
        self.row_content = new_content;
    }

    // 删除某个位置上的字符
    pub fn delete_at_position(&mut self, position: usize) {
        if position >= self.get_content_len(){
            return;
        }

        let mut new_content = String::new();
        let mut new_content_len = 0;

        // 删除position上的字符
        for (index,graheme) in self.row_content[0..].graphemes(true).enumerate(){
            // 如果当前就是要插入的位置
            if index != position {
                new_content.push_str(graheme);
                new_content_len += 1;
            }
        }
        
        self.content_len = new_content_len;
        self.row_content = new_content;
    }

    // 在某个位置上切分字符串
    pub fn split_content(&mut self, position: usize) ->Self{
        // 后半段字符串
        let mut suf_content = String::new();
        let mut suf_content_len = 0;

        // 前半段字符串
        let mut pre_content = String::new();
        let mut pre_content_len = 0;

        // 开始分割
        for (index, grapheme) in self.row_content[0..].graphemes(true).enumerate() {
            if index < position {
                pre_content_len += 1;
                pre_content.push_str(grapheme);
            } else {
                suf_content_len += 1;
                suf_content.push_str(grapheme);
            }
        }

        // 重置前半段所在行的属性
        self.row_content = pre_content;
        self.content_len = pre_content_len;
        self.is_highligthing = false;

        // 返回后半段的行
        Self{
            row_content: suf_content,
            content_len: suf_content_len,
            is_highligthing: false,
            char_display_style: Vec::new(),
            current_row:0
        }

    }

    // --------------------------
    // 搜索子字符串，未实现叠加搜索
    // --------------------------
    pub fn search_query_str(&self, query_str: &str, position: usize, direction: SearchDirection) -> Option<usize>{
        // 位置出错或字符串为空则停止搜索
        if position > self.content_len || query_str.is_empty(){
            return None;
        }

        // 搜索的起始位置
        let mut start = 0;
        if SearchDirection::Forward == direction {
            start = position;
        }else{
            start = 0;
        } 

        // 搜索的结束位置
        let mut end = self.content_len;
        if SearchDirection::Forward == direction{
            end = self.get_content_len();
        }else{
            end = position;
        }
        
        // 获取限定范围内的子串
        let substring: String = self.row_content[0..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        // 获取搜索到的起始位置
        let mut index_of_begin: Option<usize> = None;
        if SearchDirection::Forward == direction{
            index_of_begin = substring.find(query_str);
        }else{
            index_of_begin = substring.rfind(query_str);
        }

        // 如果找到了，则返回其所在原始字符串中的开始位置
        if let Some(index)  = index_of_begin{
            return  Some(index + start);
        }

        None
    }

    // 单词高亮
    fn highlight_word(&mut self,word :&Option<String>){
        if let Some(word) = word {
            if word.is_empty(){
                return;
            }

            let mut index = 0;
            while let Some(get_word_index) = self.search_query_str(word, index, SearchDirection::Forward) {
                // 如果找到一个单词，设置下一单词的查找位置
                if let Some(next_begin) = get_word_index.checked_add(word[0..].graphemes(true).count()){

                    for i in get_word_index..next_begin {
                        self.char_display_style[i] = high_lighting::Type::Match;
                    }

                    // 从新的起点开始查询
                    index = next_begin;

                }else{
                    break;
                }
            }

        }
    }

    // 子串高亮
    fn highlight_substring(
        &mut self, 
        index: &mut usize, 
        substring: &str, 
        row_content: &[char], 
        hight_type: high_lighting::Type,
    ) ->bool{
        // 如果子串为空，直接返回
        if substring.is_empty(){
            return  false;
        }

        // 比较给定的子串和chars是否匹配
        for (sub_index,c) in substring.chars().enumerate(){
            if let Some(next_char) = row_content.get(index.saturating_add(sub_index)){
                if *next_char != c{
                    return  false;
                }
            }else{
                return false;
            }
        }

        // -------------------------------------
        // 这里如果两个子串匹配，则将子串附加在后面
        // -------------------------------------
        for _index in 0..substring.len(){
            self.char_display_style.push(hight_type);
            *index += 1;
        }

        return true;
    }

    // *号指定的内容高亮
    fn highlight_astrisk(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c: char,
    ) ->bool {
        if options.asteriscs() && c == '*'{
            loop {
                self.char_display_style.push(high_lighting::Type::Asteriscs);
                *index += 1;
                if let Some(next_char) = row_content.get(*index) {
                    if *next_char == '*' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.char_display_style.push(high_lighting::Type::Asteriscs);
            *index += 1;
            return true;
        }

        false
    }

    // 关键字高亮
    fn highlight_keywords(
        &mut self, 
        index: &mut usize, 
        row_content: &[char],
        keywords: &[String],  // 关键字列表
        hight_type: high_lighting::Type, // 高亮类型
    ) ->bool{
        if *index > 0 {
            let prev_char = row_content[*index - 1];
            // 如果关键字的前面不是标点符号，证明不是关键字，不需要高亮
            if check_char(prev_char) == false{
                return  false;
            }
        }

        // 从关键字列表中依次拿出关键字
        for word in keywords{
            // 如果剩余字符长度比当前关键字长度还小就不用找了
            if *index < row_content.len().saturating_sub(word.len()){
                let next_char = row_content[*index + word.len()];
                if check_char(next_char) == false{
                    continue;
                }
            }

            // 比较子串和关键字
            if self.highlight_substring(index, &word, row_content, hight_type){
                return true;
            }
        }

        false
    }

    // 主要关键字高亮
    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
    )-> bool {
        self.highlight_keywords(index,row_content,options.primary_keywords(),high_lighting::Type::PrimaryKeywords)
    }

    // 次要关键字高亮
    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
    )-> bool{
        self.highlight_keywords(index,row_content,options.secondary_keywords(),high_lighting::Type::SecondaryKeywords)
    }

    // 数字高亮
    fn highlight_numbers(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c: char,
    ) ->bool{
        if options.numbers() && c.is_ascii_digit(){
            if *index > 0 {
                let pre_char = row_content[*index-1];
                // 如果前一个字符不是标点符号或空格证明不是想表示单独数字
                if check_char(pre_char) == false{
                    return false;
                }
            }

            loop {
                self.char_display_style.push(high_lighting::Type::Number);
                *index += 1;
                if let Some(next_char) = row_content.get(*index){
                    if *next_char != '.' && !next_char.is_ascii_digit(){
                        break;
                    }
                }else{
                    break;
                }
            }

            return true;
        }
        false
    }

    // 高亮显示''包含的字符
    fn highlight_char(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c: char,
    )-> bool{
        // 确保要求高亮显示''中的内容，且第一个字符是c
        if options.characters() && c =='\''{
            // 获取另一个'的位置
            if let Some(next_char) = row_content.get(index.saturating_add(1)){
                let end_index  = if *next_char == '\\'{
                    index.saturating_add(3)
                }else{
                    index.saturating_add(2)
                };

                // 如果end_index所指的确实是另一个'，则需要高亮显示
                if let Some(end_char) = row_content.get(end_index){
                    if *end_char == '\'' {
                        for _ in 0..=end_index.saturating_sub(*index){
                            self.char_display_style.push(high_lighting::Type::Character);
                            *index += 1;
                        }
                        return  true;
                    }
                }
            }
        }

        false
    }

    // 单行注释高亮
    fn highlight_comment(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c :char,
    ) -> bool{
        // 要求单行注释 
        if options.comments() && c== '/' && *index < row_content.len(){
            // 确保下一个也是/
            if let Some(next_char) = row_content.get(index.saturating_add(1)){
                if *next_char == '/' {
                    for _ in *index..row_content.len(){
                        self.char_display_style.push(high_lighting::Type::Comment);
                        *index += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    // --------------------------
    // 多行注释高亮,但是没有实现跨行的多行注释高亮
    // --------------------------
    fn highlight_comments(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c :char,
    )-> bool{
        // 要求高亮多行注释
        if options.comments() && c == '/' && *index < row_content.len(){
            if let Some(next_char) = row_content.get(index.saturating_add(1)){
                // 确保是以/*开头
                if *next_char == '*'{
                    // 找到以 */ 结尾的位置
                    let end_index = if let Some(end_index) = self.row_content[*index + 2..].find("*/"){
                        *index + 2 + end_index + 2
                    }else{
                        row_content.len()
                    };

                    for _ in *index..end_index{
                        self.char_display_style.push(high_lighting::Type::MultilineComment);
                        *index += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    // 字符串高亮显示
    fn highlight_string(
        &mut self,
        index: &mut usize,
        options: &HighlightingOptions,
        row_content: &[char],
        c :char,
    )-> bool{
        if options.strings() && c == '"'{
            loop {
                self.char_display_style.push(high_lighting::Type::String);
                *index += 1;

                // 如果接下来是字符
                if let Some(next_char) = row_content.get(*index){
                    if *next_char == '"'{
                        break;
                    }
                // 如果接下来不是字符
                }else{
                    break;
                }
            }

            // 处理最后一个'"'号
            self.char_display_style.push(high_lighting::Type::String);
            *index += 1;
            return true;
        }
        false
    }

    // 总体的高亮控制
    pub fn highlight_control(
        &mut self, 
        options: &HighlightingOptions, // 高亮选项
        word: &Option<String>, // 需要高亮的单词
        highlight_comment: bool, // 是否需要高亮注释
    )-> bool{  

        let row_content: Vec<char> = self.row_content.chars().collect();

        // 如果已经高亮了，且没有新单词需要高亮
        if self.is_highligthing && word.is_none(){
            if let Some(high_type) = self.char_display_style.last(){
                if *high_type == high_lighting::Type::MultilineComment &&
                self.row_content.len() > 1 &&
                self.row_content[self.row_content.len()-2..] == *"*/"{
                    return true;
                }
            }
            return false;
        }

        // 刷新每个字符的显示列表
        self.char_display_style = Vec::new();
        let mut index = 0;
        let mut high_light_comment = highlight_comment;

        // 如果要求该行直接按照高亮注释执行
        if highlight_comment{
            let end_index = if let Some(end_index) = self.row_content.find("*/"){
                end_index + 2
            }else{
                row_content.len()
            };

            // 多行注释
            for _ in 0..end_index{
                self.char_display_style.push(high_lighting::Type::MultilineComment);
            }
            index = end_index;
        }

        while let Some(c) = row_content.get(index) {
            // 先执行多行注释高亮
            if self.highlight_comments(&mut index, &options,&row_content, *c){
                high_light_comment = true;
                continue;
            }

            high_light_comment = false;

            // 其他以系列高亮选择
            if self.highlight_char(&mut index, options,&row_content, *c)
                || self.highlight_comment(&mut index, options,&row_content, *c)
                || self.highlight_primary_keywords(&mut index, &options,&row_content)
                || self.highlight_secondary_keywords(&mut index, &options,&row_content)
                || self.highlight_string(&mut index, options, &row_content, *c)
                || self.highlight_astrisk(&mut index, options, &row_content, *c)
                || self.highlight_numbers(&mut index, options, &row_content, *c)
            {
                continue;
            }

            // 如果一个字符没有匹配的，则设为默认的显示
            self.char_display_style.push(high_lighting::Type::None);
            index += 1;
        }

        self.highlight_word(word);

        if high_light_comment &&  &self.row_content[self.row_content.len().saturating_add(2)..] != "*/"{
            return true;
        }

        self.is_highligthing = true;
        false
    }

}

#[cfg(test)]
mod test_super{
    use  super::*;

    #[test]
    fn test_highlight_find() {
        let mut row = SingleRow::from("1testtest");
        row.char_display_style = vec![
            high_lighting::Type::Number,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
            high_lighting::Type::None,
        ];
        row.highlight_word(&Some("t".to_string()));
        assert_eq!(
            vec![
                high_lighting::Type::Number,
                high_lighting::Type::Match,
                high_lighting::Type::None,
                high_lighting::Type::None,
                high_lighting::Type::Match,
                high_lighting::Type::Match,
                high_lighting::Type::None,
                high_lighting::Type::None,
                high_lighting::Type::Match
            ],
            row.char_display_style
        )
    }

}