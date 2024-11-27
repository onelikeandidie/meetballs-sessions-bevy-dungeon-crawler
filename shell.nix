{ pkgs ? import <nixpkgs> {} }:
  let
    system = "x86_64";
    overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
    libPath = with pkgs; lib.makeLibraryPath [
      # load external libraries that you need in your rust project here
      udev alsa-lib vulkan-loader
      # xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
      libxkbcommon wayland # To use the wayland feature
      stdenv.cc.cc.lib
    ];
    externalLibPath = builtins.concatStringsSep ":" [
      "./fmod/api/core/lib/${system}/"
      "./fmod/api/studio/lib/${system}/"
    ];
in
  pkgs.mkShell rec {
    packages = with pkgs; [
      eza
      helix
      zellij
    ];
    nativeBuildInputs = with pkgs; [
      pkg-config
      rust-analyzer
      taplo
      lldb
      gdb
    ];
    buildInputs = with pkgs; [
      udev alsa-lib vulkan-loader
      xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
      libxkbcommon wayland # To use the wayland feature
      clang
      # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
      llvmPackages.bintools
      rustup
    ];
    RUSTC_VERSION = overrides.toolchain.channel;
    # https://github.com/rust-lang/rust-bindgen#environment-variables
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      '';
    # Add precompiled library to rustc search path
    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
      # add libraries here (e.g. pkgs.libvmi)
    ]);
    LD_LIBRARY_PATH = builtins.concatStringsSep ":" [ libPath externalLibPath];
    # LD_LIBRARY_PATH = libPath ++ [ "./game/cs_client/fmod/api/core/lib/x86_64" ];
    # Add glibc, clang, glib, and other headers to bindgen search path
    BINDGEN_EXTRA_CLANG_ARGS =
    # Includes normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # add dev libraries here (e.g. pkgs.libvmi.dev)
      pkgs.glibc.dev
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];

    # This is for nix to use RADV instead of AMDVLK since it crashes sometimes
    AMD_VULKAN_ICD="RADV";
  }
