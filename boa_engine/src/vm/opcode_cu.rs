// The opcode cost of the vm.
use super::opcode::Opcode;
use wasm_bindgen_test::__rt::js_console_log;

#[warn(unreachable_patterns)]
#[allow(clippy::match_same_arms)]
pub(crate) fn as_cost(opcode: Opcode) -> u64 {
    match opcode {
        Opcode::Pop => 2,
        Opcode::Dup => 3,
        Opcode::Swap => 3,
        Opcode::PushZero => 3,
        Opcode::PushOne => 3,
        Opcode::PushInt8 => 3,
        Opcode::PushInt16 => 3,
        Opcode::PushInt32 => 3,
        Opcode::PushRational => 3,
        Opcode::PushNaN => 3,
        Opcode::PushPositiveInfinity => 3,
        Opcode::PushNegativeInfinity => 3,
        Opcode::PushNull => 3,
        Opcode::PushTrue => 3,
        Opcode::PushFalse => 3,
        Opcode::PushUndefined => 3,
        Opcode::PushLiteral => 3,
        Opcode::PushEmptyObject => 3,
        Opcode::PushClassPrototype => 3,
        Opcode::PushNewArray => 3,
        Opcode::PushValueToArray => 3,
        Opcode::PushElisionToArray => 3,
        Opcode::PushIteratorToArray => 3,
        Opcode::Add => 3,
        Opcode::Sub => 3,
        Opcode::Div => 5,
        Opcode::Mul => 5,
        Opcode::Mod => 5,
        Opcode::Pow => 5,
        Opcode::ShiftRight => 3,
        Opcode::ShiftLeft => 3,
        Opcode::UnsignedShiftRight => 3,
        Opcode::BitOr => 3,
        Opcode::BitAnd => 3,
        Opcode::BitXor => 3,
        Opcode::BitNot => 3,
        Opcode::In => 3,
        Opcode::Eq => 3,
        Opcode::StrictEq => 3,
        Opcode::NotEq => 3,
        Opcode::StrictNotEq => 3,
        Opcode::GreaterThan => 3,
        Opcode::GreaterThanOrEq => 3,
        Opcode::LessThan => 3,
        Opcode::LessThanOrEq => 3,
        Opcode::InstanceOf => 3,
        Opcode::TypeOf => 3,
        Opcode::Void => 2,
        Opcode::LogicalNot => 3,
        Opcode::LogicalAnd => 3,
        Opcode::LogicalOr => 3,
        Opcode::Coalesce => 3,
        Opcode::Pos => 3,
        Opcode::Neg => 3,
        Opcode::Inc => 3,
        Opcode::IncPost => 3,
        Opcode::Dec => 3,
        Opcode::DecPost => 3,
        Opcode::DefInitArg => 3,
        Opcode::DefVar => 3,
        Opcode::DefInitVar => 3,
        Opcode::DefLet => 3,
        Opcode::DefInitLet => 3,
        Opcode::DefInitConst => 3,
        Opcode::GetName => 3,
        Opcode::GetNameOrUndefined => 3,
        Opcode::SetName => 3,
        Opcode::GetPropertyByName => 3,
        Opcode::GetPropertyByValue => 3,
        Opcode::SetPropertyByName => 3,
        Opcode::DefineOwnPropertyByName => 3,
        Opcode::DefineClassMethodByName => 3,
        Opcode::SetPropertyByValue => 3,
        Opcode::DefineOwnPropertyByValue => 3,
        Opcode::DefineClassMethodByValue => 3,
        Opcode::SetPropertyGetterByName => 3,
        Opcode::DefineClassGetterByName => 3,
        Opcode::SetPropertyGetterByValue => 3,
        Opcode::DefineClassGetterByValue => 3,
        Opcode::SetPropertySetterByName => 3,
        Opcode::DefineClassSetterByName => 3,
        Opcode::SetPropertySetterByValue => 3,
        Opcode::DefineClassSetterByValue => 3,
        Opcode::SetPrivateValue => 3,
        Opcode::SetPrivateSetter => 3,
        Opcode::SetPrivateGetter => 3,
        Opcode::GetPrivateField => 3,
        Opcode::PushClassComputedFieldName => 3,
        Opcode::DeletePropertyByName => 3,
        Opcode::DeletePropertyByValue => 3,
        Opcode::CopyDataProperties => 3,
        Opcode::ToPropertyKey => 3,
        Opcode::Jump => 8,
        Opcode::JumpIfFalse => 8,
        Opcode::JumpIfNotUndefined => 8,
        Opcode::Throw => 3,
        Opcode::TryStart => 3,
        Opcode::TryEnd => 3,
        Opcode::CatchStart => 3,
        Opcode::CatchEnd => 3,
        Opcode::CatchEnd2 => 3,
        Opcode::FinallyStart => 3,
        Opcode::FinallyEnd => 3,
        Opcode::FinallySetJump => 8,
        Opcode::ToBoolean => 3,
        Opcode::This => 3,
        Opcode::Case => 3,
        Opcode::Default => 3,
        Opcode::GetFunction => 3,
        Opcode::GetGenerator => 3,
        Opcode::Call => 5,
        Opcode::CallWithRest => 5,
        Opcode::New => 3,
        Opcode::NewWithRest => 3,
        Opcode::Return => 2,
        Opcode::PushDeclarativeEnvironment => 3,
        Opcode::PushFunctionEnvironment => 3,
        Opcode::PopEnvironment => 3,
        Opcode::LoopStart => 2,
        Opcode::LoopContinue => 3,
        Opcode::LoopEnd => 2,
        Opcode::ForInLoopInitIterator => 2,
        Opcode::InitIterator => 3,
        Opcode::IteratorNext => 3,
        Opcode::IteratorNextFull => 3,
        Opcode::IteratorClose => 2,
        Opcode::IteratorToArray => 5,
        Opcode::ForInLoopNext => 3,
        Opcode::ConcatToString => 3,
        Opcode::RequireObjectCoercible => 3,
        Opcode::ValueNotNullOrUndefined => 3,
        Opcode::RestParameterInit => 3,
        Opcode::RestParameterPop => 3,
        Opcode::PopOnReturnAdd => 3,
        Opcode::PopOnReturnSub => 3,
        Opcode::Yield => 3,
        Opcode::GeneratorNext => 3,
        Opcode::GeneratorNextDelegate => 3,
        Opcode::Nop => 3,
        _ => {
            js_console_log(&format!("unknown opcode: {}", &opcode.as_str()));
            3
        }
    }
}