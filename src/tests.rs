#[cfg(test)]
mod ignore_template {
    use crate::template::item::Template;
    use crate::util::http::http;
    use once_cell::sync::Lazy;
    use std::fs;

    static GITHUB_RUST_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get("https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore")
            .call()
            .map(|r| r.into_string().unwrap())
            .map(|r| r.trim().to_string())
            .unwrap()
    });
    static GITHUB_GLOBAL_LINUX_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get("https://raw.githubusercontent.com/github/gitignore/main/Global/Linux.gitignore")
            .call()
            .map(|r| r.into_string().unwrap())
            .map(|r| r.trim().to_string())
            .unwrap()
    });
    static GITHUB_COMMUNITY_OPENSSL_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get(
            "https://raw.githubusercontent.com/github/gitignore/main/community/OpenSSL.gitignore",
        )
        .call()
        .map(|r| r.into_string().unwrap())
        .map(|r| r.trim().to_string())
        .unwrap()
    });
    static GITHUB_COMMUNITY_NESTED_SNAP_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get(
        "https://raw.githubusercontent.com/github/gitignore/main/community/Linux/Snap.gitignore",
    )
        .call()
        .map(|r| r.into_string().unwrap())
        .map(|r| r.trim().to_string())
        .unwrap()
    });
    static TOPTAL_PYTHON_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get("https://www.toptal.com/developers/gitignore/api/Python")
            .call()
            .map(|r| r.into_string().unwrap())
            .map(|r| r.trim().to_string())
            .unwrap()
    });
    static TOPTAL_PYTHON_DJANGO_STACK_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get("https://www.toptal.com/developers/gitignore/api/Django")
            .call()
            .map(|r| r.into_string().unwrap())
            .map(|r| r.trim().to_string())
            .unwrap()
    });
    static TOPTAL_JETBRAINS_ALL_PATCH_TEMPLATE: Lazy<String> = Lazy::new(|| {
        http()
            .get("https://www.toptal.com/developers/gitignore/api/JetBrains+all")
            .call()
            .map(|r| r.into_string().unwrap())
            .map(|r| r.trim().to_string())
            .unwrap()
    });

    fn file_template() -> String {
        // Use this repos .gitignore file
        fs::read_to_string(".gitignore").unwrap()
    }

    #[cfg(test)]
    mod github {
        use super::*;
        #[test]
        fn no_prefix() {
            let template = Template::new("Rust").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }

        #[test]
        fn prefix() {
            let template = Template::new("gh:Rust").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod github_global {
        use super::*;

        #[test]
        fn lower_no_prefix() {
            let template = Template::new("global/Linux").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_GLOBAL_LINUX_TEMPLATE.to_string());
        }

        #[test]
        fn upper_no_prefix() {
            let template = Template::new("Global/Linux").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_GLOBAL_LINUX_TEMPLATE.to_string());
        }

        #[test]
        fn prefix() {
            let template = Template::new("ghg:Linux").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_GLOBAL_LINUX_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod github_community {
        use super::*;
        #[test]
        fn lower_no_prefix() {
            let template = Template::new("community/OpenSSL").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_OPENSSL_TEMPLATE.to_string());
        }

        #[test]
        fn upper_no_prefix() {
            let template = Template::new("Community/OpenSSL").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_OPENSSL_TEMPLATE.to_string());
        }

        #[test]
        fn prefix() {
            let template = Template::new("ghc:OpenSSL").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_OPENSSL_TEMPLATE.to_string());
        }

        #[test]
        fn nested_lower_no_prefix() {
            let template = Template::new("community/Linux/Snap").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_NESTED_SNAP_TEMPLATE.to_string());
        }

        #[test]
        fn nested_upper_no_prefix() {
            let template = Template::new("Community/Linux/Snap").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_NESTED_SNAP_TEMPLATE.to_string());
        }

        #[test]
        fn nested_prefix() {
            let template = Template::new("ghc:Linux/Snap").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_COMMUNITY_NESTED_SNAP_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod github_repo {
        use super::*;
        #[test]
        fn no_prefix() {
            let template = Template::new("github/gitignore/main/Rust.gitignore").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }

        #[test]
        fn prefix() {
            let template = Template::new("repo:github/gitignore/main/Rust.gitignore").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod toptal {
        use super::*;
        #[test]
        fn python() {
            let template = Template::new("tt:Python").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, TOPTAL_PYTHON_TEMPLATE.to_string());
        }

        #[test]
        fn python_django_stack() {
            let template = Template::new("tt:Django").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, TOPTAL_PYTHON_DJANGO_STACK_TEMPLATE.to_string());
        }

        #[test]
        fn jetbrains_all() {
            let template = Template::new("tt:JetBrains+all").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, TOPTAL_JETBRAINS_ALL_PATCH_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod url {
        use super::*;
        #[test]
        fn no_prefix() {
            let template = Template::new(
                "https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore",
            )
            .unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }

        #[test]
        fn prefix() {
            let template = Template::new(
                "url:https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore",
            )
            .unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, GITHUB_RUST_TEMPLATE.to_string());
        }
    }

    #[cfg(test)]
    mod file {
        use super::*;
        #[test]
        fn no_prefix() {
            let template = Template::new(".gitignore").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, file_template());
        }

        #[test]
        fn prefix() {
            let template = Template::new("file:.gitignore").unwrap();
            let content = template.content_body().unwrap();
            assert_eq!(content, file_template());
        }
    }
}
