pub trait PseudoTable {
    type Output;
    fn table(self, table: String) -> Self::Output;
}
