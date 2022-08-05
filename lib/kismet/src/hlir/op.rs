use super::{Exec, Instruction, SymbolTable};

/*
impl Exec1<Context> for Op {
    type Result = Value;

    fn exec(&self, c: Context) -> (Context, Self::Result) {
        fn arith_float(lhs: Float, op: &OpArith, rhs: Float) -> Value {
            Value::Primitive(Primitive::Float(match op {
                OpArith::ADD => lhs + rhs,
                OpArith::SUB => lhs - rhs,
                OpArith::MUL => lhs * rhs,
                OpArith::DIV => lhs / rhs,
                OpArith::IDIV => lhs.rem_euclid(rhs),
                OpArith::MOD => lhs % rhs,
                OpArith::POW => lhs.powf(rhs),
            }))
        }

        fn arith_int(lhs: Integer, op: &OpArith, rhs: Integer) -> Value {
            match match op {
                OpArith::ADD => lhs.checked_add(rhs),
                OpArith::SUB => lhs.checked_sub(rhs),
                OpArith::MUL => lhs.checked_mul(rhs),
                OpArith::DIV => return arith_float(lhs as Float, op, rhs as Float),
                OpArith::IDIV => lhs.checked_div(rhs),
                OpArith::MOD => lhs.checked_rem(rhs),
                OpArith::POW => match UInteger::try_from(rhs) {
                    Ok(rhs) => lhs.checked_pow(rhs),
                    Err(_) => None,
                },
            } {
                Some(x) => Value::Primitive(Primitive::Integer(x)),
                None => arith_float(lhs as Float, op, rhs as Float),
            }
        }

        match self {
            Op::And(_, _) => todo!(),
            Op::Or(_, _) => todo!(),
            Op::Not(_) => todo!(),
            Op::CompareBound {
                l_val,
                l_op,
                val,
                r_op,
                r_val,
            } => todo!(),
            Op::Compare(_, _, _) => todo!(),
            Op::Range(_) => todo!(),
            Op::Arith(lhs, op, rhs) => {
                let (c, lhs) = lhs.exec(c);
                let (c, rhs) = rhs.exec(c);
                match (lhs, rhs) {
                    (
                        Value::Primitive(Primitive::Integer(lhs)),
                        Value::Primitive(Primitive::Integer(rhs)),
                    ) => (c, arith_int(lhs, op, rhs)),
                    (
                        Value::Primitive(Primitive::Float(lhs)),
                        Value::Primitive(Primitive::Float(rhs)),
                    ) => (c, arith_float(lhs, op, rhs)),
                    (
                        Value::Primitive(Primitive::Integer(lhs)),
                        Value::Primitive(Primitive::Float(rhs)),
                    ) => (
                        c,
                        match (**op, lhs) {
                            (OpArith::POW, 2) => Value::Primitive(Primitive::Float(rhs.exp2())),
                            _ => arith_float(lhs as Float, op, rhs),
                        },
                    ),
                    (
                        Value::Primitive(Primitive::Float(lhs)),
                        Value::Primitive(Primitive::Integer(rhs)),
                    ) => (
                        c,
                        match **op {
                            OpArith::POW => Value::Primitive(Primitive::Float(lhs.powi(rhs))),
                            _ => arith_float(lhs, op, rhs as Float),
                        },
                    ),
                    _ => (c, Value::Error),
                }
            }
            Op::Unary(op, rhs) => {
                let (c, rhs) = rhs.exec(c);
                match rhs {
                    Value::Primitive(Primitive::Integer(val)) => (
                        c,
                        match **op {
                            OpArith::ADD => rhs,
                            OpArith::SUB => match val.checked_neg() {
                                Some(val) => Value::Primitive(Primitive::Integer(val)),
                                None => Value::Primitive(Primitive::Float((val as Float) * -1.)),
                            },
                            _ => Value::Error,
                        },
                    ),
                    Value::Primitive(Primitive::Float(val)) => (
                        c,
                        match **op {
                            OpArith::ADD => rhs,
                            OpArith::SUB => Value::Primitive(Primitive::Float(val * -1.)),
                            _ => Value::Error,
                        },
                    ),
                    _ => (c, Value::Error),
                }
            }
            Op::Coefficient(_, _) => todo!(),
            Op::Die(_) => todo!(),
        }
    }
}

 */
