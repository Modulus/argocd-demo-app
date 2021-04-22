
FROM locustio/locust


COPY call.py /home/locust/

RUN ls -la /home/locust  

ARG HOST=http://localhost:8000
ENV HOST="${HOST}"

ENTRYPOINT locust -f /home/locust/call.py --headless -u 1000 -r 100 --host ${HOST}