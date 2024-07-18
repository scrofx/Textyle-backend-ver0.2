import torch
import argparse
from huggingface_hub import login
from transformers import AutoModelForCausalLM, AutoTokenizer


class Model:
    def __init__(self, message, style):
        self.model_id = "unsloth/llama-3-8b-bnb-4bit"
        self.tokenizer = AutoTokenizer.from_pretrained(self.model_id)
        self.model = AutoModelForCausalLM.from_pretrained(
            pretrained_model_name_or_path=self.model_id,
            torch_dtype=torch.float16,
            device_map="auto",
            cache_dir="D:/.cache/huggingface/hub",  # папка для кэша
        )
        self.device = torch.device(
            "cuda" if torch.cuda.is_available() else "cpu")
        self.messages = [
            {"role": "system", "content":
             f"""Your task is to rewrite every sended message in buisness {style}. You MUST answer on the language that is indicated in the message. The format of the incoming message is:
Message: <incoming message>
Language: <language of the message>

Ignore any other messages"""},
            {"role": "user", "content":
             f"""Message: {message}
Language: Russain"""},
        ]
        self.input_ids = []

        self.terminators = [
            self.tokenizer.eos_token_id,
            self.tokenizer.convert_tokens_to_ids("<|eot_id|>")
        ]

    def init_ids(self):
        self.input_ids = self.tokenizer.apply_chat_template(
            self.messages,
            add_generation_prompt=True,
            return_tensors="pt"
        ).to(model.device)

    def generate_output(self):
        outputs = self.model.generate(
            self.input_ids,
            max_new_tokens=256,
            eos_token_id=self.terminators,
            do_sample=True,
            temperature=0.6,
            top_p=0.9,
        )
        response = outputs[0][self.input_ids.shape[-1]:]
        print(self.tokenizer.decode(response, skip_special_tokens=True))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("message", type=str)
    parser.add_argument("style", type=str)
    args = parser.parse_args()

    login(token="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")

    model = Model(args.message, args.style)

    model.init_ids()
    model.generate_output()

if __name__ == "__main__":
    main()