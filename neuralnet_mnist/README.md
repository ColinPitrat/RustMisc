An implementation of a neural network from scratch, used on MNIST dataset to recognize handwritten digits.
 
Retrieve data with:

```
mkdir -p data
cd data && wget http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz && gunzip train-images-idx3-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz && gunzip train-labels-idx1-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz && gunzip t10k-images-idx3-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz && gunzip t10k-labels-idx1-ubyte.gz
```

This works well with Sigmoid but for reasons that I don't manage to understand yet, this fails to work with ReLu variants.
I tried multiple things (normalizing input, lower learning rate, leaky relu with various alphas ...) but nothing seems to work.
