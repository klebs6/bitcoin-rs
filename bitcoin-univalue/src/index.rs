crate::ix!();

impl Index<&str> for UniValue {
    type Output = UniValue;
    
    #[inline] fn index(&self, key: &str) -> &Self::Output {
        todo!();
        /*
            if (typ != VOBJ)
            return NullUniValue;

        size_t index = 0;
        if (!findKey(key, index))
            return NullUniValue;

        return values.at(index);
        */
    }
}

impl Index<usize> for UniValue {
    type Output = UniValue;
    
    #[inline] fn index(&self, index: usize) -> &Self::Output {
        todo!();
        /*
            if (typ != VOBJ && typ != VARR)
            return NullUniValue;
        if (index >= values.size())
            return NullUniValue;

        return values.at(index);
        */
    }
}
