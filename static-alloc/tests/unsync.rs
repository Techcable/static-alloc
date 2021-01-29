use static_alloc::leaked::LeakBox;
use static_alloc::unsync::{Chain, MemBump};

#[test]
fn unsync_bump() {
    let chain = Chain::new(20).unwrap();

    let n1 = chain.bump_box::<u64>().unwrap();
    assert_eq!(chain.remaining_capacity(), 12);

    let n2 = chain.bump_box::<u64>().unwrap();
    assert_eq!(chain.remaining_capacity(), 4);

    let n3 = chain.bump_box::<u32>().unwrap();
    assert_eq!(chain.remaining_capacity(), 0);

    assert!(chain.bump_box::<u32>().is_err());

    chain.chain(Chain::new(40).unwrap());
    assert!(chain.bump_box::<u32>().is_ok());

    let mut n1 = LeakBox::write(n1, 10);
    let mut n2 = LeakBox::write(n2, 20);
    let mut n3 = LeakBox::write(n3, 30);

    *n1 += 1;
    *n2 += 2;
    *n3 += 3;

    assert_eq!(*n1, 11);
    assert_eq!(*n2, 22);
    assert_eq!(*n3, 33);
}

#[test]
fn bump() {
    const CAPACITY: usize = 16;
    let bump = MemBump::new(CAPACITY);
    for i in 0..CAPACITY {
        bump.get::<u8>().unwrap_or_else(|| {
            panic!("works {}", i)
        });
    }
    assert!(bump.get::<u8>().is_none());
}
