trait GenericService {
    fn send_action(name: &str, params: &[(&str, &str)]) -> SOAPError<()>;
    
    
}