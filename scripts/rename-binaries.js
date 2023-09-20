import { execa } from 'execa'
import fs from 'fs'

let extension = ''
if (process.platform === 'win32') {
  extension = '.exe'
}

async function main() {
  const rustInfo = (await execa('rustc', ['-vV'])).stdout
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
  if (!targetTriple) {
    console.error('Failed to determine platform target triple')
  }

  if (process.platform === "darwin") {
    fs.cpSync("src-tauri/binaries/wkhtmltoimage", "src-tauri/binaries/wkhtmltoimage-x86_64-apple-darwin")
    fs.cpSync("src-tauri/binaries/wkhtmltoimage", "src-tauri/binaries/wkhtmltoimage-aarch64-apple-darwin")
    fs.cpSync("src-tauri/binaries/wkhtmltopdf", "src-tauri/binaries/wkhtmltopdf-x86_64-apple-darwin")
    fs.cpSync("src-tauri/binaries/wkhtmltopdf", "src-tauri/binaries/wkhtmltopdf-aarch64-apple-darwin")

  } else if (process.platform == "win32") {

    fs.cpSync("src-tauri/binaries/wkhtmltoimage", "src-tauri/binaries/wkhtmltoimage-x86_64-pc-windows-msvc.exe")
    fs.cpSync("src-tauri/binaries/wkhtmltopdf", "src-tauri/binaries/wkhtmltopdf-x86_64-pc-windows-msvc.exe")

  } else {
    fs.renameSync(
      `src-tauri/binaries/wkhtmltoimage${extension}`,
      `src-tauri/binaries/wkhtmltoimage-${targetTriple}${extension}`,
    )
   fs.renameSync(
      `src-tauri/binaries/wkhtmltopdf${extension}`,
      `src-tauri/binaries/wkhtmltopdf-${targetTriple}${extension}`
    )
  }

  fs.rmSync("src-tauri/binaries/wkhtmltoimage")
  fs.rmSync("src-tauri/binaries/wkhtmltopdf")


}

main().catch((e) => {
  console.error(e)
})
