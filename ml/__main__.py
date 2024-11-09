import argparse
from ml import train_model, test_model

def main():
    parser = argparse.ArgumentParser(description="Train or test the model.")
    parser.add_argument('--train', action='store_true', help="Train the model")
    parser.add_argument('--test', type=str, help="Path to an image for testing")
    args = parser.parse_args()

    if args.train:
        train_model()
    elif args.test:
        test_model(args.test)
    else:
        print("Usage: python -m ml --train or python -m ml --test <image_path>")

if __name__ == "__main__":
    main()
