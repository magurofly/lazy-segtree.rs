use super::helper::LazySegtreeHelper;

use std::{cell::UnsafeCell, ops::RangeBounds};

/// https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
pub struct LazySegtree<H: LazySegtreeHelper> {
  len: usize,
  size: usize,
  log: u32,
  node: Vec<UnsafeCell<H::S>>,
  lazy: Vec<UnsafeCell<H::F>>,
}

impl<H: LazySegtreeHelper> LazySegtree<H> {
  pub fn new(len: usize) -> Self {
    let size = len.next_power_of_two();
    let mut node = Vec::with_capacity(size * 2);
    let mut lazy = Vec::with_capacity(size * 2);
    node.resize_with(size * 2, || UnsafeCell::new(H::e()));
    lazy.resize_with(size * 2, || UnsafeCell::new(H::id()));
    Self {
      len,
      size,
      log: size.trailing_zeros(),
      node,
      lazy,
    }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn set(&mut self, p: usize, x: H::S) {
    assert!(p < self.len);
    let p = p + self.size;
    for i in (1 ..= self.log).rev() {
      self.push(p >> i);
    }
    *unsafe { self.node(p) } = x;
    for i in 1 ..= self.log {
      self.update(p >> i);
    }
  }

  pub fn get(&self, p: usize) -> &H::S {
    assert!(p < self.len);
    let p = p + self.size;
    for i in (1 ..= self.log).rev() {
      self.push(p >> i);
    }
    unsafe { self.node(p) }
  }

  pub fn prod(&self, range: impl RangeBounds<usize>) -> H::S {
    let (l, r) = self.range(range);
    assert!(l <= r && r <= self.len);

    let mut l = self.size + l;
    let mut r = self.size + r;
    for i in (1 ..= self.log).rev() {
      if ((l >> i) << i) != l {
        self.push(l >> i);
      }
      if ((r >> i) << i) != r {
        self.push((r - 1) >> i);
      }
    }
    let mut sml = H::e();
    let mut smr = H::e();
    while l < r {
      if (l & 1) != 0 {
        sml = H::op(&sml, unsafe { self.node(l) });
        l += 1;
      }
      l >>= 1;
      if (r & 1) != 0 {
        r -= 1;
        smr = H::op(unsafe { self.node(r) }, &smr);
      }
      r >>= 1;
    }

    H::op(&sml, &smr)
  }

  pub fn all_prod(&self) -> &H::S {
    unsafe { self.node(1) }
  }

  pub fn apply(&mut self, p: usize, f: H::F) {
    assert!(p < self.len);
    let p = p + self.size;
    for i in (1 ..= self.log).rev() {
      self.push(p >> i);
    }
    *unsafe { self.node(p) } = H::mapping(&f, unsafe { self.node(p) });
    for i in 1 ..= self.log {
      self.update(p >> i);
    }
  }

  pub fn apply_range(&mut self, range: impl RangeBounds<usize>, f: H::F) {
    let (l, r) = self.range(range);
    assert!(l <= r && r <= self.len);
    if l == r {
      return;
    }

    let mut l = l + self.size;
    let mut r = r + self.size;

    for i in (1 ..= self.log).rev() {
      if ((l >> i) << i) != l {
        self.push(l >> i);
      }
      if ((r >> i) << i) != r {
        self.push((r - 1) >> i);
      }
    }

    {
      let l2 = l;
      let r2 = r;
      while l < r {
        if (l & 1) != 0 {
          self.all_apply(l, &f);
          l += 1;
        }
        l >>= 1;
        if (r & 1) != 0 {
          r -= 1;
          self.all_apply(r, &f);
        }
        r >>= 1;
      }
      l = l2;
      r = r2;
    }

    for i in 1 ..= self.log {
      if ((l >> i) << i) != l {
        self.update(l >> i);
      }
      if ((r >> i) << i) != r {
        self.update((r - 1) >> i);
      }
    }
  }

  pub fn max_right<P: FnMut(&H::S) -> bool>(&self, l: usize, mut predicate: P) -> usize {
    assert!(l <= self.len);
    assert!(predicate(&H::e()));
    if l == self.len {
      return self.len;
    }
    let mut l = l + self.size;
    for i in (1 ..= self.log).rev() {
      self.push(l >> i);
    }
    let mut sm = H::e();
    loop {
      l >>= l.trailing_zeros();
      if !predicate(&H::op(&sm, unsafe { self.node(l) })) {
        while l < self.size {
          self.push(l);
          l <<= 1;
          if predicate(&H::op(&sm, unsafe { self.node(l) })) {
            sm = H::op(&sm, unsafe { self.node(l) });
            l += 1;
          }
        }
        return l - self.size;
      }
      sm = H::op(&sm, unsafe { self.node(l) });
      l += 1;
      if (l & !l) == l {
        break;
      }
    }
    self.len
  }

  pub fn min_left<P: FnMut(&H::S) -> bool>(&self, r: usize, mut predicate: P) -> usize {
    assert!(r <= self.len);
    assert!(predicate(&H::e()));
    if r == 0 {
      return 0;
    }
    let mut r = r + self.size;
    for i in (1 ..= self.log).rev() {
      self.push((r - 1) >> i);
    }
    let mut sm = H::e();
    loop {
      r -= 1;
      while r > 1 && (r & 1) != 0 {
        r >>= 1;
      }
      if !predicate(&H::op(unsafe { self.node(r) }, &sm)) {
        while r < self.size {
          self.push(r);
          r = 2 * r + 1;
          if predicate(&H::op(unsafe { self.node(r) }, &sm)) {
            sm = H::op(unsafe { self.node(r) }, &sm);
            r -= 1;
          }
        }
        return r + 1 - self.size;
      }
      sm = H::op(unsafe { self.node(r) }, &sm);
      if (r & !r) == r {
        break;
      }
    }
    0
  }

  fn range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
    use std::ops::Bound::*;
    let l = match range.start_bound() {
      Included(&x) => x,
      Excluded(&x) => x + 1,
      Unbounded => 0,
    };
    let r = match range.end_bound() {
      Included(&x) => x + 1,
      Excluded(&x) => x,
      Unbounded => self.len,
    };
    (l, r)
  }

  unsafe fn node(&self, i: usize) -> &mut H::S {
    &mut *(&self.node[i]).get()
  }

  unsafe fn lazy(&self, i: usize) -> &mut H::F {
    &mut *(&self.lazy[i]).get()
  }

  fn update(&self, k: usize) {
    unsafe {
      *self.node(k) = H::op(&self.node(2 * k), &self.node(2 * k + 1));
    }
  }

  fn all_apply(&self, k: usize, f: &H::F) {
    unsafe {
      *self.node(k) = H::mapping(f, self.node(k));
      if k < self.size {
        *self.lazy(k) = H::composition(f, self.lazy(k));
        // https://rsm9.hatenablog.com/entry/2021/02/01/220408
        if H::is_failed(self.node(k)) {
          self.push(k);
          self.update(k);
        }
      }
    }
  }

  fn push(&self, k: usize) {
    unsafe {
      self.all_apply(2 * k, self.lazy(k));
      self.all_apply(2 * k + 1, self.lazy(k));
      *self.lazy(k) = H::id();
    }
  }
}

impl<H: LazySegtreeHelper, I: IntoIterator<Item = H::S> + ExactSizeIterator> From<I> for LazySegtree<H> {
  fn from(iter: I) -> Self {
    let len = iter.len();
    let size = len.next_power_of_two();
    let mut node = Vec::with_capacity(size * 2);
    unsafe {
      node.set_len(size * 2);
      node[0] = UnsafeCell::new(H::e());
    }
    for (i, x) in iter.into_iter().enumerate() {
      node[size + i] = UnsafeCell::new(x);
    }
    for i in len .. size {
      node[size + i] = UnsafeCell::new(H::e());
    }
    for i in (1 .. size).rev() {
      node[i] = UnsafeCell::new(unsafe { H::op(&*node[i * 2].get(), &*node[i * 2 + 1].get()) });
    }
    let mut lazy = Vec::with_capacity(size * 2);
    lazy. resize_with(size * 2, || UnsafeCell::new(H::id()));
    Self {
      len,
      size,
      log: size.trailing_zeros(),
      node,
      lazy,
    }
  }
}

impl<H: LazySegtreeHelper> std::ops::Index<usize> for LazySegtree<H> {
  type Output = H::S;
  fn index(&self, i: usize) -> &Self::Output {
    self.get(i)
  }
}