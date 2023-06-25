pub trait Analysis {
    type Output;
}

#[derive(Debug, Clone)]
pub enum Input {
    Tran(Tran),
    Ac(Ac),
}

#[derive(Debug, Clone)]
pub enum Output {
    Tran(TranOutput),
    Ac(AcOutput),
}

#[derive(Debug, Clone)]
pub struct Spectre;
#[derive(Debug, Clone)]
pub struct Tran;
#[derive(Debug, Clone)]
pub struct Ac;
#[derive(Debug, Clone)]
pub struct TranOutput;
#[derive(Debug, Clone)]
pub struct AcOutput;

impl Analysis for Tran {
    type Output = TranOutput;
}
impl Analysis for Ac {
    type Output = AcOutput;
}

impl Supports<Tran> for Spectre {
    fn into_input(a: Tran, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Tran(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Tran as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Tran(tran) => tran,
            _ => panic!("tran analysis output did not get back tran output"),
        }
    }
}
impl Supports<Ac> for Spectre {
    fn into_input(a: Ac, inputs: &mut Vec<Self::Input>) {
        inputs.push(Input::Ac(a));
    }
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> <Ac as Analysis>::Output {
        let output = outputs.next().unwrap();
        match output {
            Output::Ac(ac) => ac,
            _ => panic!("ac analysis output did not get back ac output"),
        }
    }
}

pub trait Supports<A: Analysis>: Simulator {
    fn into_input(a: A, inputs: &mut Vec<Self::Input>);
    fn from_output(outputs: &mut impl Iterator<Item = Self::Output>) -> A::Output;
}

pub trait SupportedBy<S: Simulator>: Analysis {
    fn into_input(self, inputs: &mut Vec<S::Input>);
    fn from_output(outputs: &mut impl Iterator<Item = S::Output>) -> Self::Output;
}

impl<S, A> SupportedBy<S> for A
where
    A: Analysis,
    S: Supports<A>,
{
    fn into_input(self, inputs: &mut Vec<<S as Simulator>::Input>) {
        S::into_input(self, inputs);
    }
    fn from_output(outputs: &mut impl Iterator<Item = <S as Simulator>::Output>) -> Self::Output {
        S::from_output(outputs)
    }
}

pub trait Simulator {
    type Input;
    type Output;
    fn raw_simulate(&self, input: Vec<Self::Input>) -> Vec<Self::Output>;

    fn simulate<A: Analysis>(&self, input: A) -> A::Output
    where
        Self: Supports<A>,
        A: Analysis,
        Self: Sized,
    {
        let mut inputs = Vec::new();
        Self::into_input(input, &mut inputs);
        let output = self.raw_simulate(inputs);
        let mut output = output.into_iter();
        Self::from_output(&mut output)
    }
}

impl<T1, T2> Analysis for (T1, T2)
where
    T1: Analysis,
    T2: Analysis,
{
    type Output = (T1::Output, T2::Output);
}

impl<T1, T2, S> Supports<(T1, T2)> for S
where
    S: Supports<T1> + Supports<T2>,
    T1: Analysis,
    T2: Analysis,
    S: Simulator,
{
    fn into_input(a: (T1, T2), inputs: &mut Vec<S::Input>) {
        Self::into_input(a.0, inputs);
        Self::into_input(a.1, inputs);
    }

    fn from_output(
        outputs: &mut impl Iterator<Item = Self::Output>,
    ) -> <(T1, T2) as Analysis>::Output {
        let o0 = <Self as Supports<T1>>::from_output(outputs);
        let o1 = <Self as Supports<T2>>::from_output(outputs);
        (o0, o1)
    }
}

impl Simulator for Spectre {
    type Input = Input;
    type Output = Output;

    fn raw_simulate(&self, input: Vec<Input>) -> Vec<Output> {
        let mut outputs = Vec::new();
        for input in input {
            match input {
                Input::Tran(_) => outputs.push(Output::Tran(TranOutput)),
                Input::Ac(_) => outputs.push(Output::Ac(AcOutput)),
            }
        }
        outputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simulate_tuple() {
        let simulator = Spectre;
        let x = simulator.simulate((Tran, Ac));
        println!("Result of simulating (Tran, Ac): {x:?}");
    }
}
