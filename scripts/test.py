import deepnano2
import os
import numpy as np

def med_mad(x, factor=1.4826):
    """
    Calculate signal median and median absolute deviation
    """
    med = np.median(x)
    print("median: " + str(med))
    mad = np.median(np.absolute(x - med)) * factor
    print("MAD: " + str(mad))
    return med, mad

def rescale_signal(signal):
    signal = signal.astype(np.float32)
    med, mad = med_mad(signal)
    signal -= med
    signal /= mad
    return signal

network_type = "48"
beam_size = 5
beam_cut_threshold = 0.01
weights = os.path.join(deepnano2.__path__[0], "weights", "rnn%s.txt" % network_type)
caller = deepnano2.Caller(network_type, weights, beam_size, beam_cut_threshold)

# Minimal size for calling is STEP*3 + PAD*6 + 1 (STEP and PAD are defined in src/lib.rs)
signal = np.random.normal(size=(1000*3+10*6+1))
np.savetxt('test.out', signal, delimiter=',', fmt='%10.10f')

signal = rescale_signal(signal)
for i in signal:
    print(str(i) + "\n")

print(caller.call_raw_signal(signal))