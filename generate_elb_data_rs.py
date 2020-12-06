import json

# Via https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-access-logs.html#access-log-entry-syntax
spec = """
proto
time
elb
client:port
target:port
request_processing_time
target_processing_time
response_processing_time
elb_status_code
target_status_code
received_bytes
sent_bytes
"request"
"user_agent"
ssl_cipher
ssl_protocol
target_group_arn
"trace_id"
"domain_name"
"chosen_cert_arn"
matched_rule_priority
request_creation_time
"actions_executed"
"redirect_url"
"error_reason"
"target:port_list"
"target_status_code_list"
"classification"
"classification_reason"
""".strip().splitlines()
regex_bits = []
fields = []
for line in spec:
    quoted = line.startswith('"')
    line = line.strip('"').replace(":", "_")
    if quoted:
        regex_bits.append(fr'"(?P<{line}>.+?)"')
    else:
        regex_bits.append(fr"(?P<{line}>[^\s]+)")
    fields.append(line)
regex = "^" + " ".join(regex_bits)

template = f"""
use regex::{{Regex}};
pub const LINE_RE_TEXT: &str = {json.dumps(regex)};
pub const FIELD_NAMES: [&str; {len(fields)}] = {json.dumps(fields)};
lazy_static! {{
    pub static ref LINE_RE: Regex = Regex::new(LINE_RE_TEXT).unwrap();
}}
""".strip()

print(template)
