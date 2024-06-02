function reverseWordPrefix(word: string, ch: string): string {
  if (word.includes(ch)) {
    const first = word.split(ch, 1)[0];

    return ch + first.split("").reverse().join("") + word.replace(first + ch, "");
  }

  return word;
}

console.log(reverseWordPrefix("xyxzxe", 'z'));
