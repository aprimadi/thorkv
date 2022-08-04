fn main() {
    let db = thorkv::db::DB::open("db");
    db.put("user_id", "1").unwrap();
}
