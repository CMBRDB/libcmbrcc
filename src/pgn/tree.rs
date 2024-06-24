#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T> {
    // NOTE: Using a whole ass vec here feels a bit icky
    // TODO(#5): (MAYBE) Replace this with a smaller vec, or make the user suply a vec-like type
    children: Vec<Box<Tree<T>>>,
    value: T,
}

#[allow(dead_code)]
impl<T> Tree<T> {
    #[inline(always)]
    pub fn new() -> Self {
        return Tree {
            children: Vec::new(),
            value: unsafe { std::mem::zeroed() },
        };
    }

    #[inline(always)]
    pub fn new_with_value(v: T) -> Self {
        return Tree {
            children: Vec::new(),
            value: v,
        };
    }

    #[inline(always)]
    pub fn append(&mut self, v: T) {
        self.children.push(Box::new(Self::new_with_value(v)));
    }

    #[inline(always)]
    pub fn insert(&mut self, v: T) -> &mut Box<Tree<T>> {
        self.append(v);

        self.children.last_mut().unwrap()
    }

    #[inline(always)]
    pub fn get_value(&self) -> &T {
        return &self.value;
    }

    #[inline(always)]
    pub fn get_value_mut(&mut self) -> &mut T {
        return &mut self.value;
    }

    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        return self.children.len() == 0;
    }

    #[inline(always)]
    pub fn get_children(&self) -> &Vec<Box<Tree<T>>> {
        return &self.children;
    }

    #[inline(always)]
    pub fn get_children_mut(&mut self) -> &mut Vec<Box<Tree<T>>> {
        return &mut self.children;
    }
}
