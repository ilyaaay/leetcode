function reverseString(s: string[]): void {
  let length = s.length - 1;
  let i = 0;

  while (length > i) {
    const x = s[i];
    s[i] = s[length];
    s[length] = x;
    length -= 1;
    i += 1;
  }
}

const test = ['h', 'e', 'l', 'l', 'o'];

reverseString(test);

// ['o', 'l', 'l', 'e', 'h']
