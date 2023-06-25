pub trait Analysis {
    type Output;
}

pub enum Input {
    Tran(Tran),
    Ac(Ac),
}

pub enum Output {
    Tran(TranOutput),
    Ac(AcOutput),
}

pub struct Spectre;
pub struct Tran;
pub struct Ac;
pub struct TranOutput;
pub struct AcOutput;

impl Analysis for Tran {
    type Output = TranOutput;
}

impl SupportedBy<Spectre> for Tran {
    fn into_input(self, inputs: &mut Vec<<Spectre as Simulator>::Input>) {
        todo!()
    }
    fn from_output(outputs: &mut Vec<<Spectre as Simulator>::Output>) -> Self::Output {
        todo!()
    }
}

pub trait SupportedBy<S: Simulator>: Analysis {
    fn into_input(self, inputs: &mut Vec<S::Input>);
    fn from_output(outputs: &mut Vec<S::Output>) -> Self::Output;
}

pub trait Simulator {
    type Input;
    type Output;
    fn raw_simulate(&self, input: Vec<Self::Input>) -> Vec<Self::Output>;

    fn simulate<A: Analysis>(&self, input: A) -> A::Output
    where
        A: SupportedBy<Self>,
        Self: Sized,
    {
        let mut inputs = Vec::new();
        input.into_input(&mut inputs);
        let mut output = self.raw_simulate(inputs);
        A::from_output(&mut output)
    }
}

impl<T1, T2> Analysis for (T1, T2)
where
    T1: Analysis,
    T2: Analysis,
{
    type Output = (T1::Output, T2::Output);
}

impl<T1, T2, S> SupportedBy<S> for (T1, T2)
where
    T1: SupportedBy<S>,
    T2: SupportedBy<S>,
    S: Simulator,
{
    fn into_input(self, inputs: &mut Vec<S::Input>) {
        todo!()
    }

    fn from_output(output: &mut Vec<S::Output>) -> Self::Output {
        todo!()
    }
}

pub trait SimulatorExt: Simulator {
    fn simulate<A: Analysis>(&self, input: A) -> A::Output;

    // fn simulate1<T1>(&self, input: T1) -> T1::Output where T1: Analysis, T1: Into<Self::Input>, Self::Output: Into<T1::Output> {
    //     self.simulate(vec![input.into()]).pop().unwrap().into()
    // }
}

impl Simulator for Spectre {
    type Input = Input;
    type Output = Output;

    fn raw_simulate(&self, input: Vec<Input>) -> Vec<Output> {
        todo!()
    }
}

mod tests {
    use super::*;
    #[test]
    fn simulate_tuple() {
        let simulator = Spectre;
        let x = simulator.simulate((Tran, Tran));
        println!("hi");
    }
}
