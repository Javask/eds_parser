use crate::eds_file::Address;

#[test]
pub fn test_order(){
    let a= Address::new(1000, 0);
    let b =Address::new(1000, 1);
    let c =Address::new(1001, 0);
    let d =Address::new(1001, 1);
    assert_eq!(a,a);
    assert_eq!(b,b);
    assert_eq!(c,c);
    assert_eq!(d,d);
    assert!(a<b);
    assert!(a<c);
    assert!(a<d);
    assert!(b<c);
    assert!(b<d);
    assert!(c<d);
}