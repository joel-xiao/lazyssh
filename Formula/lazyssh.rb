class Lazyssh < Formula
  desc "A cross-platform SSH management tool with TUI interface"
  homepage "https://github.com/joel-xiao/lazyssh"
  url "https://github.com/joel-xiao/lazyssh/archive/v0.2.0.tar.gz"
  sha256 "" # Replace with actual SHA256 after first release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/lazyssh", "--help"
  end
end

