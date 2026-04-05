class Lsmd < Formula
  desc "lsmd — List Markdown, a markdown-aware directory listing tool"
  homepage "https://github.com/leaf-kit/ls.md"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/leaf-kit/ls.md/releases/download/v0.1.0/lsmd-aarch64-apple-darwin.tar.gz"
      sha256 "3098a97632c8331d51eb79e508acbcc71eb3cc0409ae8f79b4952eddc305a8ee"
    else
      url "https://github.com/leaf-kit/ls.md/releases/download/v0.1.0/lsmd-x86_64-apple-darwin.tar.gz"
      sha256 "617e9fa68a4d766e39756013d5a9f79eb5778cb0f67d296f66a35b8b77a162a4"
    end
  end

  def install
    bin.install "lsmd"
  end

  test do
    assert_match "lsmd", shell_output("#{bin}/lsmd --version")
  end
end
