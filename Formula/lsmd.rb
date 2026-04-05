class Lsmd < Formula
  desc "lsmd — List Markdown, a markdown-aware directory listing tool"
  homepage "https://github.com/leaf-kit/ls.md"
  url "https://github.com/leaf-kit/ls.md/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "c358eb2630fa5f9259324df725e9fd5865c27d43c46a6b540e113655558c7701"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lsmd", shell_output("#{bin}/lsmd --version")
  end
end
