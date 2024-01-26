fn main() {
    let mut collection = vec![1, 2, 3];

    // iter iterates over the collection, giving you a reference to each
    // element. for_each consumes the iterator in a for loop.
    collection.iter().for_each(|item: &i32| {
        println!("item = {:?}", item);
    });
    println!("{:?}", collection);
    // iter_mut is like iter, iterating over the collection and giving you
    // a reference to each element, but the reference is mutable. Useful
    // if you want to change the collection in place. You need to already
    // have a mutable collection to call this.
    collection.iter_mut().for_each(|item: &mut i32| {
        *item += 1;
        println!("item = {:?}", item);
    });

    // into_iter consumes the collection, so it passes you ownership of
    // each element. Great for if you want to avoid unnecessary copies,
    // and this is the last thing you need to do with your source list.
    collection.into_iter().for_each(|item: i32| {
        println!("item = {:?}", item);
    });

    /* Separate */

    let src = vec![1, 2, 3];

    let mut dest: Vec<String> = Vec::new();
    for item in &src {
        // The essence of map is that for each item in your source list,
        // you're writing something into your destination list.
        dest.push(format!("{} doubled is {}", item, item * 2));
    }

    // The same format expression appears here, but it's in a call to
    // map. The collect at the end takes the iterator and collects it into
    // a Vec.
    // (more details on collect later in this article)
    let dest_with_map: Vec<String> = src
        .iter()
        .map(|item: &i32| format!("{} doubled is {}", item, item * 2))
        .collect();

    println!("dest = {:?}", dest);
    println!("dest_with_map = {:?}", dest_with_map);

    /* Separate */

    let src = vec![1, 2, 3, 4];

    let mut dest: Vec<i32> = Vec::new();
    for item in &src {
        // In the imperative form, flat_map translates to a nested for
        // loop.
        for range_item in 0..*item {
            dest.push(range_item);
        }
    }

    let dest_with_flatmap: Vec<i32> = src.iter().flat_map(|item: &i32| 0..*item).collect();

    println!("dest = {:?}", dest);
    println!("dest_with_flatmap = {:?}", dest_with_flatmap);

    /* Separate */

    let src = vec!["one", "1", "3", "not a number"];

    let mut dest: Vec<i32> = Vec::new();
    for item in &src {
        // This is a lot like a regular map, followed by a filter.
        let maybe_parsed: Option<i32> = item.parse().ok();
        if let Some(parsed) = maybe_parsed {
            dest.push(parsed);
        }
    }

    let dest_with_filter_map: Vec<i32> = src
        .iter()
        .filter_map(|item: &&str| item.parse().ok())
        .collect();

    println!("dest = {:?}", dest);
    println!("dest_with_filter_map = {:?}", dest_with_filter_map);

    /* Separate */

    let src = vec![1, 2, 3, 4, 5, 6];

    // When you think "fold", think "accumulator variable"
    let mut sum: i32 = 0;
    for item in &src {
        sum = sum + item
    }

    // This is functionally equivalent to
    // 0 + 1 + 2 + 3 + 4 + 5 + 6
    // (the 0 is important in case the list is empty)
    // It's a common convention to call the accumulator `acc`
    let sum_with_fold: i32 = src.iter().fold(0, |acc: i32, item: &i32| acc + item);

    // Summing is a common use case, so there's a fold shorthand just for
    // that in the standard library.
    let sum_with_sum: i32 = src.iter().sum();

    // Notice that we don't have a `collect` on these, because `fold` is
    // already consuming the iterator into the type that we want to end up
    // with.

    println!("sum = {:?}", sum);
    println!("sum_with_fold = {:?}", sum_with_fold);
    println!("sum_with_sum = {:?}", sum_with_sum);

    /* Separate */

    let src = vec![1, 2, 3, 4, 5, 6];

    let any_with_fold: bool = src
        .iter()
        .fold(false, |acc: bool, item: &i32| acc || *item > 3);

    let any_with_any: bool = src.iter().any(|item: &i32| *item > 3);

    println!("any_with_fold = {:?}", any_with_fold);
    println!("any_with_any = {:?}", any_with_any);
    // true

    let all_with_fold: bool = src
        .iter()
        .fold(true, |acc: bool, item: &i32| acc && *item > 3);

    let all_with_all: bool = src.iter().all(|item: &i32| *item > 3);

    println!("all_with_fold = {:?}", all_with_fold);
    println!("all_with_all = {:?}", all_with_all);
    // false

    /* Separate */

    let src = vec![1, 2, 3, 4, 5, 6];

    let mut found: Option<&i32> = None;
    for item in &src {
        // This is a lot like a filter that stops after the first result.
        if *item > 3 {
            found = Some(item);
            break;
        }
    }

    let found_with_find: Option<&i32> = src.iter().find(|item: &&i32| **item > 3);

    println!("found = {:?}", found);
    println!("found_with_find = {:?}", found_with_find);

    /* Separate */

    // This is a very standard linked list implementation for sake of
    // example. Each element in the list is either the empty list (Nil),
    // or a single item and a pointer (Box) to the rest of the list
    // (Cons).
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // We can then implement FromIterator, specifying the type of thing the
    // iterator needs to be going over. In this case, we can take an
    // iterator of i32s, and make our List of i32s. This can also be
    // generic.
    impl std::iter::FromIterator<i32> for List {
        // IntoIterator means it has the into_iter() function. If the
        // thing is already an iterator, like in all of the examples using
        // `collect` above, then into_iter() is a no-op which will be
        // optimized away by the compiler.
        fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> List {
            iter.into_iter().fold(List::Nil, |acc: List, next: i32| {
                dbg!(&acc);
                List::Cons(next, Box::new(acc))
            })
        }
    }

    let src = vec![1, 2, 3];

    // Having implemented FromIterator, we can collect into a List. The
    // only difference from the examples above, where we collected into a
    // Vec, is the type annotation on our variable.
    let list: List = src.iter().map(|item: &i32| item * 2).collect();

    println!("list = {:?}", list);

    /* Separate */

    let src_numbers = vec![1, 2, 3];
    let src_words = vec!["one", "two", "three"];

    let mut dest: Vec<String> = Vec::new();

    for i in 0..src_numbers.len().min(src_words.len()) {
        // Indexing into two lists can be particularly error prone. What
        // if one happens to be longer than the other, for example?
        let num_item: i32 = src_numbers[i];
        let word_item: &str = src_words[i];
        dest.push(format!("{}: {}", num_item, word_item));
    }

    // Notice how we can chain iterators, using zip to combine the two but
    // after that it's just a normal map and collect.
    let dest_with_zip: Vec<String> = src_numbers
        .iter()
        .zip(src_words.iter())
        .map(|(num_item, word_item): (&i32, &&str)| format!("{}: {}", num_item, word_item))
        .collect();

    println!("dest = {:?}", dest);
    println!("dest_with_zip = {:?}", dest_with_zip);

    /* Separate */

    let src1 = vec![1, 2, 3];
    let src2 = vec![4, 5, 6];

    let mut dest: Vec<i32> = Vec::new();
    // Without chain, you need this function body listed out twice.
    for item in &src1 {
        dest.push(item * 2);
    }
    for item in &src2 {
        dest.push(item * 2);
    }

    // This version separates the composing of the iterators from the map
    // that you wanted to do to them.
    let dest_with_chain: Vec<i32> = src1
        .iter()
        .chain(src2.iter())
        .map(|item: &i32| item * 2)
        .collect();

    println!("dest = {:?}", dest);
    println!("dest_with_chain = {:?}", dest_with_chain);

}
