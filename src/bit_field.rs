pub struct BitField {
    inner: Vec<usize>,
    len: usize,
}

impl BitField {
    const SIZE: usize = usize::BITS as usize;

    pub fn new() -> Self {
        Self::init(1024)
    }
    pub fn init(len: usize) -> Self {
        let len = len.div_ceil(Self::SIZE);
        Self {
            inner: vec![0; len],
            len,
        }
    }
    pub fn reserve_init(&mut self, len: usize) {
        let len = len.div_ceil(Self::SIZE);
        self.inner.reserve_exact(len);

        for _ in 0..len {
            self.inner.push(0);
        }
    }
    // Panics if not reserved
    pub fn set(&mut self, index: usize, value: bool) {
        let index = index.div_ceil(Self::SIZE);
        let remainder = index % Self::SIZE;
        let bytes = &mut self.inner[index];
        *bytes = *bytes & !((1 << remainder) | ((value as usize) << remainder));
    }
    // Panics if not reserved
    pub fn get(&mut self, index: usize) -> bool {
        let index = index.div_ceil(Self::SIZE);
        let remainder = index % Self::SIZE;

        self.inner[index] >> remainder == 1
    }
    pub fn iter_fields(&self) -> impl Iterator<Item = Field> + '_ {
        self.inner.iter().map(|x| Field { inner: *x })
    }
}

impl Default for BitField {
    fn default() -> Self {
        Self::init(0)
    }
}

pub struct Field {
    inner: usize,
}

pub struct FieldIterator {
    field: usize,
    index: usize,
}

impl Iterator for FieldIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < usize::BITS as usize {
            let value = self.index << 1 == 1;
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
