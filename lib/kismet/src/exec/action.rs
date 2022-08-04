use super::{Exec, Instruction, SymbolTable};

pub struct Call<F, T, U, V> {
    pub args: Vec<Instruction<T, U, V>>,
    pub action: F,
}

impl<F, T, U, V> Exec<SymbolTable<U>, (SymbolTable<U>, V)> for Call<F, T, U, V>
where
    F: Exec<Vec<V>, V>,
    T: Exec<SymbolTable<U>, (SymbolTable<U>, V)>,
    V: From<U> + Clone + Default,
    U: From<V> + Clone + Default,
{
    fn exec(&self, i: SymbolTable<U>) -> (SymbolTable<U>, V) {
        let (i, args) = self.args.iter().fold((i, vec![]), |(i, mut vec), val| {
            let (i, val) = val.exec(i);
            vec.push(val);
            (i, vec)
        });
        (i, self.action.exec(args))
    }
}
