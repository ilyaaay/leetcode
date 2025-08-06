function two_sum_first(nums, target) {
  const indeces_map = new Map(nums.entries().map(([idx, x]) => [x, idx]));

  for (let i = 0; i < nums.length; i++) {
    const idx = indeces_map.get(target - nums[i]);

    if (idx !== undefined && i !== idx) {
      return [i, idx];
    }
  }

  return [];
}

function two_sum_second(nums, target) {
  const indeces_map = new Map();

  for (let i = 0; i < nums.length; i++) {
    const idx = indeces_map.get(target - nums[i]);

    if (idx !== undefined) {
      return [idx, i];
    }

    indeces_map.set(nums[i], i);
  }

  return [];
}

