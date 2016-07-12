use modinv::ModInv;
use modmult::ModMult;
use std::ops::{Add, Mul};
use std::fmt::Debug;
use num::{Zero, One, Integer};


#[derive(Clone, Debug)]
pub struct Mod<I> where I: Clone + Debug {
    value: I,
    modulus: I,
}

impl<I> Mod<I> where I: Debug + Clone + Eq + ModInv<I> + Integer {
    fn new(value: &I, modulus: &I) -> Mod<I> {
        Mod {
            value: value.mod_floor(modulus),
            modulus: modulus.clone(),
        }
    }

    fn un_mod(&self) -> I {
        self.value.clone()
    }

    fn inv(&self) -> Option<Mod<I>> {
        let result = self.value.mod_inv(&self.modulus);
        match result {
            Some(new_val) => Some(Mod::new(&new_val, &self.modulus)),
            None          => None,
        }
    }

    fn zero(modulus: &I) -> Mod<I> {
        Mod::new(&Zero::zero(), modulus)
    }

    fn one(modulus: &I) -> Mod<I> {
        Mod::new(&One::one(), modulus)
    }
}

impl<I> Add<Mod<I>> for Mod<I> 
    where I: Clone + Eq + Debug + Add<I, Output=I> + ModInv<I> + Integer 
{
    type Output = Mod<I>;

    fn add(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.value, rhs.value);

        let new_value = (self.value + rhs.value).mod_floor(&self.modulus);
        
        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Add<&'a Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn add(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.value, rhs.value);

        let new_value = (self.value + &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Add<Mod<I>> for &'a Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn add(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (rhs.value + &self.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, 'b, I> Add<&'a Mod<I>> for &'b Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer,
          &'b I: Add<&'a I, Output=I>
{
    type Output = Mod<I>;

    fn add(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (&self.value + &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<I> Mul<Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Mul<I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);
        
        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Mul<&'a Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Mul<Mod<I>> for &'a Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, 'b, I> Mul<&'a Mod<I>> for &'b Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer,
          &'b I: ModMult
{
    type Output = Mod<I>;

    fn mul(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

#[cfg(test)]
mod tests {

} 