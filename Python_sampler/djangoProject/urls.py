from django.urls import path
from .api.views import resize_images

urlpatterns = [
    path('api/resize/', resize_images, name='resize_images'),
]
