use regex::Regex;
pub const LINE_RE_TEXT: &str = "^(?P<proto>[^\\s]+) (?P<time>[^\\s]+) (?P<elb>[^\\s]+) (?P<client_port>[^\\s]+) (?P<target_port>[^\\s]+) (?P<request_processing_time>[^\\s]+) (?P<target_processing_time>[^\\s]+) (?P<response_processing_time>[^\\s]+) (?P<elb_status_code>[^\\s]+) (?P<target_status_code>[^\\s]+) (?P<received_bytes>[^\\s]+) (?P<sent_bytes>[^\\s]+) \"(?P<request>.+?)\" \"(?P<user_agent>.+?)\" (?P<ssl_cipher>[^\\s]+) (?P<ssl_protocol>[^\\s]+) (?P<target_group_arn>[^\\s]+) \"(?P<trace_id>.+?)\" \"(?P<domain_name>.+?)\" \"(?P<chosen_cert_arn>.+?)\" (?P<matched_rule_priority>[^\\s]+) (?P<request_creation_time>[^\\s]+) \"(?P<actions_executed>.+?)\" \"(?P<redirect_url>.+?)\" \"(?P<error_reason>.+?)\" \"(?P<target_port_list>.+?)\" \"(?P<target_status_code_list>.+?)\" \"(?P<classification>.+?)\" \"(?P<classification_reason>.+?)\"";
pub const FIELD_NAMES: [&str; 29] = [
    "proto",
    "time",
    "elb",
    "client_port",
    "target_port",
    "request_processing_time",
    "target_processing_time",
    "response_processing_time",
    "elb_status_code",
    "target_status_code",
    "received_bytes",
    "sent_bytes",
    "request",
    "user_agent",
    "ssl_cipher",
    "ssl_protocol",
    "target_group_arn",
    "trace_id",
    "domain_name",
    "chosen_cert_arn",
    "matched_rule_priority",
    "request_creation_time",
    "actions_executed",
    "redirect_url",
    "error_reason",
    "target_port_list",
    "target_status_code_list",
    "classification",
    "classification_reason",
];
lazy_static! {
    pub static ref LINE_RE: Regex = Regex::new(LINE_RE_TEXT).unwrap();
}
