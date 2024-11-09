import argparse
import os
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, Dataset
from torchvision import datasets, transforms, models

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
MODEL_PATH = os.path.join(BASE_DIR, "model.pth")
DATA_DIR = os.path.join(BASE_DIR, "data")
transform = transforms.Compose([
    transforms.Resize((64, 64)),
    transforms.ToTensor(),
    transforms.Normalize([0.485, 0.456, 0.406], [0.229, 0.224, 0.225])
])

def train_model():
    dataset = datasets.ImageFolder(root=DATA_DIR, transform=transform)
    dataloader = DataLoader(dataset, batch_size=8, shuffle=True)

    model = models.resnet18(pretrained=True)
    model.fc = nn.Linear(model.fc.in_features, len(dataset.classes))

    criterion = nn.CrossEntropyLoss()
    optimizer = optim.SGD(model.parameters(), lr=0.001, momentum=0.9)

    for _ in range(1):  # Quick training for testing
        for inputs, labels in dataloader:
            optimizer.zero_grad()
            outputs = model(inputs)
            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()

    torch.save(model.state_dict(), "ml/model.pth")
    print("Training complete. Model saved as ml/model.pth.")

def test_model(image_path, model_path=None):
    """
    Test the model by classifying the given image.
    """
    if model_path is None:
        model_path = MODEL_PATH

    model = models.resnet18(pretrained=False)
    model.fc = nn.Linear(model.fc.in_features, 4)
    model.load_state_dict(torch.load(model_path))
    model.eval()

    image = datasets.folder.default_loader(image_path)
    image = transform(image).unsqueeze(0)

    with torch.no_grad():
        output = model(image)
        _, predicted = torch.max(output, 1)

    class_names = ['cloudy', 'desert', 'green_area', 'water']
    print(f"Predicted class: {class_names[predicted.item()]}")
    return class_names[predicted.item()]