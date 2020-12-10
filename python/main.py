from prometheus_client import start_http_server, Summary, Gauge, Counter, exposition
import random
import time
import resource



# Create a metric to track time spent and requests made.
REQUEST_TIME = Summary('request_processing_seconds', 'Time spent processing request')

# Decorate function with metric.
@REQUEST_TIME.time()
def process_request(t):
    """A dummy function that takes some time."""
    time.sleep(t)



if __name__ == '__main__':
    counters = []
    for i in range(2000):
        g = Gauge("counter{0}".format(i), "help", ['number', 'foo'])
        counters.append([i,g])
    print("After creating counters: ", resource.getrusage(0).ru_maxrss)

    for i, c in counters:
       c.labels(str(i), 'bar').set(41.2)

    # Start up the server to expose the metrics.
    start_http_server(8080)
    # Generate some requests.
    while True:
        process_request(random.random())
