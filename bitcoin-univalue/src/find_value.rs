crate::ix!();

pub fn find_value<'a>(
        obj:  &'a UniValue,
        name: &'a str) -> &'a UniValue {
    
    todo!();
        /*
            for (unsigned int i = 0; i < obj.keys.size(); i++)
            if (obj.keys[i] == name)
                return obj.values.at(i);

        return NullUniValue;
        */
}

pub fn find_value_mut<'a>(
        obj:  &'a mut UniValue,
        name: &'a str) -> &'a mut UniValue {
    
    todo!();
        /*
            for (unsigned int i = 0; i < obj.keys.size(); i++)
            if (obj.keys[i] == name)
                return obj.values.at(i);

        return NullUniValue;
        */
}
