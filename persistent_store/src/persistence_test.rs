#[cfg(test)]
mod tests {
    use crate::proto::StoreSnapshot;
    use crate::store::{Store, Value};
    use prost::Message;
    use std::collections::HashMap;

    #[test]
    fn protobuf_roundtrip_preserves_values() {
        let mut data = HashMap::new();
        data.insert("age".to_string(), Value::Integer(30));
        data.insert("name".to_string(), Value::Text("Francesco".to_string()));
        data.insert("active".to_string(), Value::Boolean(true));
        data.insert("score".to_string(), Value::Float(3.5));

        let store = Store::from_data(data);

        let proto_store = store.store_to_proto_store();
        let bytes = proto_store.encode_to_vec();

        let decoded_proto =
            StoreSnapshot::decode(bytes.as_slice()).expect("Should decode protobuf bytes");

        let decoded_store =
            Store::proto_to_store(decoded_proto).expect("Should convert proto store to Store");

        match decoded_store.get_value("age") {
            Some(Value::Integer(n)) => assert_eq!(*n, 30),
            _ => panic!("Expected integer value"),
        }

        match decoded_store.get_value("name") {
            Some(Value::Text(s)) => assert_eq!(s, "Francesco"),
            _ => panic!("Expected text value"),
        }

        match decoded_store.get_value("active") {
            Some(Value::Boolean(b)) => assert_eq!(*b, true),
            _ => panic!("Expected boolean value"),
        }

        match decoded_store.get_value("score") {
            Some(Value::Float(n)) => assert_eq!(*n, 3.5),
            _ => panic!("Expected float value"),
        }
    }
}
