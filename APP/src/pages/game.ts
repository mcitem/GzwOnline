function getImageDimensions(
  imageUrl: string,
): Promise<{ width: number, height: number }> {
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.src = imageUrl

    img.onload = () => {
      resolve({
        width: img.width,
        height: img.height,
      })
    }

    img.onerror = (err) => {
      reject(err)
    }
  })
}
function shuffleArray(
  array: Array<number>,
  array2: Array<Array<number>>,
): Array<number> {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]]
    for (let k = 0; k < 4; k++) {
      [array2[i][k], array2[j][k]] = [array2[j][k], array2[i][k]]
    }
  }
  return array
}

export { getImageDimensions, shuffleArray }
