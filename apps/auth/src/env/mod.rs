use std::env;

pub struct Environment;

impl Environment {
    pub fn get_elasticsearch_api() -> String {
        env::var("ELASTICSEARCH_API").unwrap_or("".to_string())
    }
    pub fn get_chat_table_name() -> String {
        env::var("CHAT_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_history_table_name() -> String {
        env::var("HISTORY_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_prompt_table_name() -> String {
        env::var("PROMPT_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_default_prompt_table_name() -> String {
        env::var("DEFAULT_PROMPT_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_config_table_name() -> String {
        env::var("CONFIG_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_default_config_table_name() -> String {
        env::var("DEFAULT_CONFIG_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_opentelemetry_server_url() -> String {
        env::var("OPENTELEMETRY_SERVER_URL").unwrap_or("".to_string())
    }

    pub fn get_tianxin_embedding_table_name() -> String {
        env::var("TIANXIN_EMBEDDING_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_embedding_table_name() -> String {
        env::var("EMBEDDING_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_xml_table_name() -> String {
        env::var("MOYUEUDEMON_XML_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_xml_pin_table_name() -> String {
        env::var("MOYUEUDEMON_XML_PIN_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_test_question_table_name() -> String {
        env::var("MOYUEUDEMON_TEST_QUESTION_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_test_answer_table_name() -> String {
        env::var("MOYUEUDEMON_TEST_ANSWER_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_embedding_api() -> String {
        env::var("EMBEDDING_API").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_api() -> String {
        env::var("MOYUEUDEMON_API").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_chats_table_name() -> String {
        env::var("MOYUEUDEMON_CHATS_TABLE_NAME").unwrap_or("".to_string())
    }
    pub fn get_moyueudemon_node_label_table_name() -> String {
        env::var("MOYUEUDEMON_NODE_LABEL_TABLE_NAME").unwrap_or("".to_string())
    }
}
