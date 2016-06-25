import os
import numpy as np
import tensorflow as tf

with tf.Session() as session:
    a = tf.Variable(42.0, name='a')
    b = tf.Variable(69.0, name='b')
    c = tf.mul(a, b, name='c')
    directory = os.path.dirname(os.path.realpath(__file__))
    tf.train.write_graph(session.graph_def, directory, 'graph.pb', as_text=False)
