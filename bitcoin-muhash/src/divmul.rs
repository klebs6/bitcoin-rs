crate::ix!();

impl MulAssign<&MuHash3072> for MuHash3072 {
    
    /**
      | Multiply (resulting in a hash for the
      | union of the sets)
      |
      */
    #[inline] fn mul_assign(&mut self, mul: &MuHash3072) {
        todo!();
        /*
            m_numerator.Multiply(mul.m_numerator);
        m_denominator.Multiply(mul.m_denominator);
        return *this;
        */
    }
}

impl DivAssign<&MuHash3072> for MuHash3072 {
    
    /**
      | Divide (resulting in a hash for the difference
      | of the sets)
      |
      */
    #[inline] fn div_assign(&mut self, div: &MuHash3072) {
        todo!();
        /*
            m_numerator.Multiply(div.m_denominator);
        m_denominator.Multiply(div.m_numerator);
        return *this;
        */
    }
}
