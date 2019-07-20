An implementation of a neural network from scratch, used on MNIST dataset to recognize handwritten digits.
 
Retrieve data with:

```
mkdir -p data
cd data && wget http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz && gunzip train-images-idx3-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz && gunzip train-labels-idx1-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz && gunzip t10k-images-idx3-ubyte.gz
cd data && wget http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz && gunzip t10k-labels-idx1-ubyte.gz
```
