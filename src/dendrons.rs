use std::collections::HashMap;
use std::{collections::BTreeMap, fmt::Display};
use clap::builder::Str;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::cmp::{Ord,Ordering};
use std::ops::Div;
use itertools::Itertools;

#[derive(Hash,PartialEq,Eq,Clone,Debug)]
struct Exp(Dendron);

impl Ord for Exp{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for Exp{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Exp{
    fn mul(left : Exp, right : &Exp) -> Exp{
        let mut exponent = left.0;
        exponent.add_assign(&right.0);
        Exp(exponent)
    }
    fn is_one(&self) -> bool{
        self.0.is_zero()
    }
    fn one()->Exp{
        Exp(Dendron::zero())
    }

    fn try_divide(&self, divisor : &Exp) -> Option<Exp>{
        if let Some(delta) = self.0.try_subtract(&divisor.0){
            Some(Exp(delta))
        } else {None}
    }

    fn as_tex(&self) -> String{
        if self.0.is_zero(){
            "1".to_string()
        } else if self.0 == Dendron::exp(Dendron::zero()) {
            r"\omega".to_string()
        } else {
            format!(r"\omega^{{{}}}",self.0.as_tex())
        }
    }
}

impl Display for Exp{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_one(){return write!(f,"1");}
        write!(f,"[{}]",self.0)
    }
}

// #[derive(Debug)]
// struct Term{
//     exp : Exp,
//     coeff : BigUint
// }
// impl Display for Term{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"{}{}")
//     }
// }


#[derive(Eq,PartialEq,Clone,Debug, Hash)]
pub struct Dendron{
    terms : BTreeMap<Exp,BigUint>
}


impl Ord for Dendron{
    fn cmp(&self, other: &Self) -> Ordering {
        let self_revit = self.terms.iter().rev();
        let other_revit = other.terms.iter().rev();
        match self_revit
        .zip_longest(other_revit)
        .try_for_each(|either_or_both|{
            let default = (&Exp::one(),&BigUint::zero());
            let((se,sc),(oe,oc)) 
                = either_or_both.or(default,default);
            let ordering = se.cmp(&oe).then(
                sc.cmp(&oc)
            );
            match ordering{
                Ordering::Equal => Ok(()),
                _ => Err(ordering)
            }
        }){
            Ok(()) => Ordering::Equal,
            Err(ord) => ord
        }        
    }
}
impl PartialOrd for Dendron{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Dendron{
    pub fn is_zero(&self) -> bool{
        self.terms.len() == 0
    }

    pub fn is_one(&self) -> bool{
        if self.terms.len() != 1 {
            false
        } else {
            self.terms.first_key_value().unwrap()
            == (&Exp::one(),&BigUint::one())
        }
    }

    
    #[allow(dead_code)]
    fn from_terms_nonunique(terms : Vec<(Exp,BigUint)>) -> Dendron{
        // let mut unique_terms = HashMap::new();

        // terms.into_iter().for_each(|(e,c)|{
        //     match unique_terms.get_mut(&e){
        //         Some(cell) => *cell += c,
        //         None => {unique_terms.insert(e, c);},
        //     }
        // });
        // Dendron::from_terms(unique_terms.into_iter().collect())
        let mut d = Dendron::zero();
        terms.into_iter().for_each(|(e,c)|{
            d.add_assign_monomial(&e, c);
        });
        d
    }

    fn add_assign_monomial(&mut self, exponential : &Exp, coefficient : BigUint){
        match self.terms.get_mut(&exponential){
            None => {self.terms.insert(exponential.clone(), coefficient);},
            Some(coeff) => *coeff += coefficient
        }
    }

    pub fn add_assign(&mut self, other : &Dendron){
        if self.is_zero(){
            *self = other.clone();
            return;
        }
        if other.is_zero(){
            return;
        };
        other.terms.iter().for_each(|(exp,coeff)|{
            self.add_assign_monomial(exp, coeff.clone());
        })
    }

    pub fn mul(left : Dendron, right : Dendron) -> Dendron{
        let mut accumulator = Dendron::zero();
        left.terms.iter()
        .cartesian_product(right.terms.iter())
        .for_each(|((el,cl),(er,cr))|{
            
            let prod = Exp::mul(el.clone(),&er);
            // println!("{} x {} -> {}",el,er,prod);
            accumulator.add_assign_monomial(&prod, cl*cr)
        });
        accumulator
    }

    pub fn zero() -> Dendron{
        Dendron { terms: BTreeMap::new() }
    }

    pub fn exp(exponent : Dendron) -> Dendron{
        let mut terms = BTreeMap::new();
        let _ = terms.insert(Exp(exponent),BigUint::one());
        Dendron{ terms}
        
    }

    pub fn from_int(n : BigUint) -> Dendron{
        if n.is_zero() {
            Dendron::zero()
        } else {
            let mut terms = BTreeMap::new();
            let _ = terms.insert(Exp::one(),n);
            Dendron{
                terms
            }
        }
    }

    fn fit(&self, other : &Dendron) -> BigUint{
        assert_ne!(*other, Dendron::zero());

        let zero = &BigUint::zero();
        other.terms.iter().map(|(e,c)|{
            let oc = self.terms.get(e).unwrap_or(zero);
            oc.div(c)
        })
        .min().unwrap()
    }

    fn scale(de : Dendron, factor : BigUint) -> Dendron{
        let mut de = de;
        de.terms.iter_mut().for_each(|(_,c)|*c *= factor.clone());
        de
    }

    pub fn divsub(&mut self, pi : &Dendron, sigma : &Dendron){
        let mut psi = Dendron::zero();
        let mut rho = self.clone();
        loop {
            let mut found = false;
            for ((se,sc),(pe,pc)) in rho.terms.iter()
            .cartesian_product(pi.terms.iter())
            {
                match se.try_divide(pe){
                    None => {},
                    Some(exp_delta) => {
                        if sc >= pc{
                            let exp_delta_d = Dendron::from(exp_delta);
                            
                            let bit = Dendron::mul(pi.clone(),exp_delta_d.clone());
                            let n = rho.fit(&bit);
                            
                            assert!(n>BigUint::zero());

                            // println!("We can fit {} a number n={} of times",bit,n);

                            rho = rho.try_subtract(&Dendron::scale(bit,n.clone())).unwrap();
                            psi.add_assign(&Dendron::scale(exp_delta_d,n));

                            found = true;
                            break;
                        }
                    }
                }
            };
            if !found {break;}
        }

        rho.add_assign(&Dendron::mul(psi, sigma.clone()));
        *self = rho;
    }

    

    fn try_decrement_coeff(&mut self, exp : &Exp, amount : BigUint) -> Option<()>{
        assert!(!amount.is_zero());

        if let Some(coeff) = self.terms.get_mut(&exp){
            if *coeff >= amount {
                *coeff -= amount;
                if coeff.is_zero(){
                    let _ = self.terms.remove(exp);
                }
                return Some(())
            }
        };

        None
    }

    pub fn try_decrement(&mut self) -> Option<()>{
        self.try_decrement_coeff(&Exp::one(), BigUint::one())
    }


    fn add(left : Dendron,right :&Dendron) -> Dendron{
        let mut left = left;
        left.add_assign(&right);
        left
    }

    pub fn sum(dendrons : Vec<Dendron>) -> Dendron{
        dendrons.into_iter().reduce(|a,b|Dendron::add(a, &b)).unwrap_or(Dendron::zero())
    }

    fn try_subtract(&self, other : &Dendron) -> Option<Dendron>{
        let does_subtract = other.terms.iter().map(|(e,c)|{
            match self.terms.get(e){
                None => false,
                Some(oc) => oc >= c
            }
        }).reduce(|a,b|a&b).unwrap_or(true);
        if does_subtract{
            let mut acc = self.clone();
            other.terms.iter().for_each(|(e,c)|{
                acc.try_decrement_coeff(e, c.clone()).unwrap()
            });
            Some(acc)
        } else {
            None
        }
    }

    pub fn move_finite(&mut self, destination : &Dendron){
        assert!(!destination.terms.contains_key(&Exp::one()));
        let multiplier = 
            if let Some(v) = self.terms.remove(&Exp::one())
            {v} else {
                return
            };
        if !destination.is_zero(){
            self.add_assign(
                &Dendron::scale(
                    destination.clone(),
                    multiplier)
            );
        }
    }

    fn as_finite(&self) -> Option<BigUint>{
        if self.is_zero() {
            return Some(BigUint::zero())
        }
        for (e,c) in self.terms.iter(){
            if e.is_one(){
                return Some(c.clone());
            };
            return None
        };
        unreachable!()
        
    }

    pub fn as_c_str(&self) -> Option<String>{
        
        let mut char_map : HashMap<usize, u8> = HashMap::new();
        for (e,c) in self.terms.iter(){
            match e.0.as_finite(){
                Some(n_bu) => {
                    if let Ok(c) = c.try_into(){
                        let n : usize = n_bu.try_into().unwrap();
                        char_map.insert(n, c);
                    } else {
                        return None;
                    }
                },
                None => return None
            }
        };

        let mut buffer = Vec::new();
        char_map.iter().for_each(|(idx, val)|{
            while buffer.len() < idx+1 {
                buffer.push(0);
            }
            buffer[*idx] = *val;
        });

        std::str::from_utf8(&buffer).ok()
        .map(|s|s.to_owned() )

    }

    pub fn as_tex(&self)->String{
        if self.is_zero(){ return "0".to_owned()};

        self.terms.iter().map(|(e,c)|{
            if e.is_one(){
                c.to_string()
            } else if c.is_one() {
                e.as_tex()
            } else{
                format!("{}{}",c.to_string(),e.as_tex())
            }
        }).join("+")
    }
}

// impl FromStr for Dendron{
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
        
//     }
// }
impl From<Exp> for Dendron{
    fn from(value: Exp) -> Self {
        let mut map = BTreeMap::new();
        map.insert(value, BigUint::one());
        Dendron{terms:map}

    }
}


impl Display for Dendron{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero(){
            return write!(f,"0");
        }

        write!(f,"{}",
            self.terms
            .iter().rev().map(|(e,c)| 
                if e.is_one(){
                    format!("{}",c)
                } else{
                    if *c == BigUint::one(){
                        format!("{}",e)
                    } else {
                        format!("{}{}",c,e)
                    }
                    
                }

                
            ).join("+")
        )
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_dendrons(){
        let zero = Dendron::zero();
        assert!(zero.is_zero());
        let one = Dendron::exp(zero);
        
        let two = Dendron::add(one.clone(),&one);
        let three = Dendron::add(one.clone(),&two);
        let threeb = Dendron::add(two.clone(),&one);
        let threec = Dendron::from_int(BigUint::from(3u32));

        assert_eq!(three,threeb);
        assert_eq!(threeb,threec);
        assert_eq!(three,threec);

        assert_ne!(three,two);

        let omega = Dendron::exp(one.clone());
        let omega2 = Dendron::mul(omega.clone(),omega.clone());

        println!("{} squared is {}",omega,omega2);

        assert_eq!(omega2,Dendron::exp(Dendron::from_int(2u32.into())));

        let omega_plus_one = Dendron::add(omega.clone(), &one);
        let omega_plus_oneb = Dendron::add(one.clone(),&Dendron::exp(one.clone()));

        assert_eq!(omega_plus_one,omega_plus_oneb);
        println!("{}",omega_plus_oneb);

        let square = Dendron::mul(omega_plus_one.clone(),omega_plus_oneb);

        
        let desired_square = Dendron::from_terms_nonunique(vec![
            (Exp(two.clone()), BigUint::one()),
            (Exp(one.clone()), BigUint::from(2u32)),
            (Exp::one(), BigUint::one())
        ]);

        println!("{} vs {}",square,desired_square);
        assert_eq!(square,desired_square);

        assert_ne!(omega_plus_one,Dendron::exp(one.clone()));
        

        assert_eq!(omega.try_subtract(&one), None);
        assert_eq!(omega_plus_one.try_subtract(&one), Some(omega));

        assert_eq!(omega2.try_subtract(&omega2),Some(Dendron::zero()));

        assert_eq!(desired_square.try_subtract(&Dendron::zero()).unwrap(),desired_square);

        let omega = Dendron::exp(Dendron::exp(Dendron::zero()));
        assert_eq!(Exp(omega.clone()).try_divide(&Exp::one()).unwrap(),Exp(omega));
    }
}