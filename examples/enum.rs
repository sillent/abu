use zabbix_api;

fn main() {
    let k = zabbix_api::methods::Item::Create;
    println!("{:?}", k);

}
