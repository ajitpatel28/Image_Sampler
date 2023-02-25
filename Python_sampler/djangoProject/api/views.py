import numpy as np
import cv2
import os
from django.http import HttpResponse
from django.shortcuts import render
from django.views.decorators.csrf import csrf_exempt
import time


@csrf_exempt
def resize_images(request):
    if request.method == 'POST':
        # Get the user input from the form
        input_images = request.FILES.getlist('images')
        width = int(request.POST.get('width'))
        height = int(request.POST.get('height'))

        if not os.path.exists('result'):
            os.makedirs('result')

        start_time = time.time()
        for i, img in enumerate(input_images):
            image = cv2.imdecode(np.fromstring(img.read(), np.uint8), cv2.IMREAD_COLOR)

            # Resize the image using linear interpolation
            resized_image = cv2.resize(image, (width, height), interpolation=cv2.INTER_LINEAR)

            filename = f'result/resized_{i}.jpg'
            cv2.imwrite(filename, resized_image)

        end_time = time.time()
        elapsed_time = end_time - start_time

        return HttpResponse(f'Images resized and saved successfully! Elapsed time: {elapsed_time:.2f} seconds')

    else:
        return render(request, 'resize_form.html')
