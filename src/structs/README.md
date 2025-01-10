# Implementation Notes

Implementation Notes exists as a place to put implentation notes as development goes on.


## 0.1.0

### Vectors
Vec<T> are being used of T* to not have to deal with unsafety in the early stages. It would make sense to make Safe Variants first, then institute unsafe variants that can be moved into safe versions to deal with rust safety descrepancy between C/C++.

### Lifetimes
It is really hard to predict the optimal lifetime structure for references on Nodes and Skeletons without beginning to write actual conversion algorithms. Usually, you would just use a pointer and assume that cleanup will be handled by owning structure since the deconstruction of the scene will always be done at the top-level first, but with keeping to rust's safety, it is hard for me to figure out the exact structure of lifetimes, both due to inexperience with them and lack of future sight. I imagine that they will be a place of consistent return.

### Materials

Inside AiMaterials, I encoded AiPropertyTypeInfo + Data as an ADT enum instead of a fieldless enum and a bytes. This is meant to make working with AiMaterials simpler as we can easily do pattern-matching for the different variants and be complete. Originally, I tried to retain a fieldless enum and byte-based data to copy assimp, but this ends up providing more complex syntax that could interact strangely in edge scenarios relating to encoding and decoding.