class Lsmd < Formula
  desc "lsmd — List Markdown, a markdown-aware directory listing tool"
  homepage "https://github.com/leaf-kit/ls.md"
  url "https://github.com/leaf-kit/ls.md/archive/refs/tags/v0.2.1.tar.gz"
  sha256 "083d26d1dea0931e3b3412ed5055cfbe803877f7a75a5caffc4054e117c35055"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lsmd", shell_output("#{bin}/lsmd --version")
  end
end
