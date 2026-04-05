class Lsmd < Formula
  desc "lsmd — List Markdown, a markdown-aware directory listing tool"
  homepage "https://github.com/leaf-kit/ls.md"
  url "https://github.com/leaf-kit/ls.md/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "e328fe9aeb71f9ecb9ac09766c453f456ca6ac39c6f1e6e4db43782763a93ee5"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lsmd", shell_output("#{bin}/lsmd --version")
  end
end
