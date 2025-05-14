// ---------------- [ File: bitcoin-fuzz/src/fuzz_prevector.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/prevector.cpp]

pub struct PrevectorTester<T: Default,const N: usize> {
    real_vector:     <Self as HasPrevectorTesterTypes>::RealType,
    real_vector_alt: <Self as HasPrevectorTesterTypes>::RealType,
    pre_vector:      <Self as HasPrevectorTesterTypes>::PreType,
    pre_vector_alt:  <Self as HasPrevectorTesterTypes>::PreType,
}

pub trait HasPrevectorTesterTypes {
    type RealType;
    type PreType;
}

impl<T: Default,const N: usize> HasPrevectorTesterTypes for PrevectorTester<T,N> {
    type RealType = Vec<T>;
    type PreType  = PreVector<T,N>;
}

impl<T: Default,const N: usize> PrevectorTester<T,N> {

    pub fn test(&self)  {
        
        todo!();
        /*
            const pretype& const_pre_vector = pre_vector;
            assert(real_vector.size() == pre_vector.size());
            assert(real_vector.empty() == pre_vector.empty());
            for (usize s = 0; s < real_vector.size(); s++) {
                assert(real_vector[s] == pre_vector[s]);
                assert(&(pre_vector[s]) == &(pre_vector.begin()[s]));
                assert(&(pre_vector[s]) == &*(pre_vector.begin() + s));
                assert(&(pre_vector[s]) == &*((pre_vector.end() + s) - real_vector.size()));
            }
            // assert(realtype(pre_vector) == real_vector);
            assert(pretype(real_vector.begin(), real_vector.end()) == pre_vector);
            assert(pretype(pre_vector.begin(), pre_vector.end()) == pre_vector);
            size_t pos = 0;
            for (const T& v : pre_vector) {
                assert(v == real_vector[pos]);
                ++pos;
            }
            for (const T& v : reverse_iterate(pre_vector)) {
                --pos;
                assert(v == real_vector[pos]);
            }
            for (const T& v : const_pre_vector) {
                assert(v == real_vector[pos]);
                ++pos;
            }
            for (const T& v : reverse_iterate(const_pre_vector)) {
                --pos;
                assert(v == real_vector[pos]);
            }
            DataStream ss1(SER_DISK, 0);
            DataStream ss2(SER_DISK, 0);
            ss1 << real_vector;
            ss2 << pre_vector;
            assert(ss1.size() == ss2.size());
            for (usize s = 0; s < ss1.size(); s++) {
                assert(ss1[s] == ss2[s]);
            }
        */
    }
    
    pub fn resize(&mut self, s: usize)  {
        
        todo!();
        /*
            real_vector.resize(s);
            assert(real_vector.size() == s);
            pre_vector.resize(s);
            assert(pre_vector.size() == s);
        */
    }
    
    pub fn reserve(&mut self, s: usize)  {
        
        todo!();
        /*
            real_vector.reserve(s);
            assert(real_vector.capacity() >= s);
            pre_vector.reserve(s);
            assert(pre_vector.capacity() >= s);
        */
    }
    
    pub fn insert_pos_value(&mut self, 
        position: usize,
        value:    &T)  {
        
        todo!();
        /*
            real_vector.insert(real_vector.begin() + position, value);
            pre_vector.insert(pre_vector.begin() + position, value);
        */
    }
    
    pub fn insert(&mut self, 
        position: usize,
        count:    usize,
        value:    &T)  {
        
        todo!();
        /*
            real_vector.insert(real_vector.begin() + position, count, value);
            pre_vector.insert(pre_vector.begin() + position, count, value);
        */
    }
    
    
    pub fn insert_range<I>(&mut self, 
        position: usize,
        first:    I,
        last:     I)  {
    
        todo!();
        /*
            real_vector.insert(real_vector.begin() + position, first, last);
            pre_vector.insert(pre_vector.begin() + position, first, last);
        */
    }
    
    pub fn erase_pos(&mut self, position: usize)  {
        
        todo!();
        /*
            real_vector.erase(real_vector.begin() + position);
            pre_vector.erase(pre_vector.begin() + position);
        */
    }
    
    pub fn erase(&mut self, 
        first: usize,
        last:  usize)  {
        
        todo!();
        /*
            real_vector.erase(real_vector.begin() + first, real_vector.begin() + last);
            pre_vector.erase(pre_vector.begin() + first, pre_vector.begin() + last);
        */
    }
    
    pub fn update(&mut self, 
        pos:   usize,
        value: &T)  {
        
        todo!();
        /*
            real_vector[pos] = value;
            pre_vector[pos] = value;
        */
    }
    
    pub fn push_back(&mut self, value: &T)  {
        
        todo!();
        /*
            real_vector.push_back(value);
            pre_vector.push_back(value);
        */
    }
    
    pub fn pop_back(&mut self)  {
        
        todo!();
        /*
            real_vector.pop_back();
            pre_vector.pop_back();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            real_vector.clear();
            pre_vector.clear();
        */
    }
    
    pub fn assign(&mut self, 
        n:     usize,
        value: &T)  {
        
        todo!();
        /*
            real_vector.assign(n, value);
            pre_vector.assign(n, value);
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return real_vector.size();
        */
    }
    
    pub fn capacity(&self) -> usize {
        
        todo!();
        /*
            return pre_vector.capacity();
        */
    }
    
    pub fn shrink_to_fit(&mut self)  {
        
        todo!();
        /*
            pre_vector.shrink_to_fit();
        */
    }
    
    pub fn swap(&mut self)  {
        
        todo!();
        /*
            real_vector.swap(real_vector_alt);
            pre_vector.swap(pre_vector_alt);
        */
    }
    
    pub fn move_(&mut self)  {
        
        todo!();
        /*
            real_vector = std::move(real_vector_alt);
            real_vector_alt.clear();
            pre_vector = std::move(pre_vector_alt);
            pre_vector_alt.clear();
        */
    }
    
    pub fn copy_(&mut self)  {
        
        todo!();
        /*
            real_vector = real_vector_alt;
            pre_vector = pre_vector_alt;
        */
    }
    
    pub fn resize_uninitialized(&mut self, values: <Self as HasPrevectorTesterTypes>::RealType)  {
        
        todo!();
        /*
            size_t r = values.size();
            size_t s = real_vector.size() / 2;
            if (real_vector.capacity() < s + r) {
                real_vector.reserve(s + r);
            }
            real_vector.resize(s);
            pre_vector.resize_uninitialized(s);
            for (auto v : values) {
                real_vector.push_back(v);
            }
            auto p = pre_vector.size();
            pre_vector.resize_uninitialized(p + r);
            for (auto v : values) {
                pre_vector[p] = v;
                ++p;
            }
        */
    }
}

#[fuzz_test] fn prevector() {
    todo!();
    /*
        FuzzedDataProvider prov(buffer.data(), buffer.size());
        prevector_tester<8, int> test;

        LIMITED_WHILE(prov.remaining_bytes(), 3000)
        {
            switch (prov.ConsumeIntegralInRange<int>(0, 13 + 3 * (test.size() > 0))) {
            case 0:
                test.insert(prov.ConsumeIntegralInRange<size_t>(0, test.size()), prov.ConsumeIntegral<int>());
                break;
            case 1:
                test.resize(std::max(0, std::min(30, (int)test.size() + prov.ConsumeIntegralInRange<int>(0, 4) - 2)));
                break;
            case 2:
                test.insert(prov.ConsumeIntegralInRange<size_t>(0, test.size()), 1 + prov.ConsumeBool(), prov.ConsumeIntegral<int>());
                break;
            case 3: {
                int del = prov.ConsumeIntegralInRange<int>(0, test.size());
                int beg = prov.ConsumeIntegralInRange<int>(0, test.size() - del);
                test.erase(beg, beg + del);
                break;
            }
            case 4:
                test.push_back(prov.ConsumeIntegral<int>());
                break;
            case 5: {
                int values[4];
                int num = 1 + prov.ConsumeIntegralInRange<int>(0, 3);
                for (int k = 0; k < num; ++k) {
                    values[k] = prov.ConsumeIntegral<int>();
                }
                test.insert_range(prov.ConsumeIntegralInRange<size_t>(0, test.size()), values, values + num);
                break;
            }
            case 6: {
                int num = 1 + prov.ConsumeIntegralInRange<int>(0, 15);
                std::vector<int> values(num);
                for (auto& v : values) {
                    v = prov.ConsumeIntegral<int>();
                }
                test.resize_uninitialized(values);
                break;
            }
            case 7:
                test.reserve(prov.ConsumeIntegralInRange<size_t>(0, 32767));
                break;
            case 8:
                test.shrink_to_fit();
                break;
            case 9:
                test.clear();
                break;
            case 10:
                test.assign(prov.ConsumeIntegralInRange<size_t>(0, 32767), prov.ConsumeIntegral<int>());
                break;
            case 11:
                test.swap();
                break;
            case 12:
                test.copy();
                break;
            case 13:
                test.move();
                break;
            case 14:
                test.update(prov.ConsumeIntegralInRange<size_t>(0, test.size() - 1), prov.ConsumeIntegral<int>());
                break;
            case 15:
                test.erase(prov.ConsumeIntegralInRange<size_t>(0, test.size() - 1));
                break;
            case 16:
                test.pop_back();
                break;
            }
        }

        test.test();

    */
}
