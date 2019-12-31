pub trait Node<Input, Output> {
    fn process(&mut self, input: Input) -> Output;
}

pub trait ReadableNode<Output> {
    fn read(&self) -> Output;
}
