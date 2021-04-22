import time
import random
from locust import HttpUser, task, between


def get_color(num):

    if num == 1:
        return "green"
    elif num == 2:
        return "yellow"
    else:
        return "red"

class QuickstartUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def index_page(self):
        pass

    @task(3)
    def view_item(self):
        for item_id in range(1000):
            num = random.randint(1,3)
            color = get_color(num)
            self.client.post(f"/vote/{color}")
            time.sleep(1)

    def on_start(self):
        pass