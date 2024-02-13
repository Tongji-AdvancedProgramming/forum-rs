# Details

Date : 2024-02-12 10:14:30

Directory /Users/cineazhan/codelab/67ForumSystemNew/forum-rs

Total : 88 files,  8818 codes, 260 comments, 1180 blanks, all 10258 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [.idea/dataSources.xml](/.idea/dataSources.xml) | XML | 19 | 0 | 0 | 19 |
| [.idea/forum.iml](/.idea/forum.iml) | XML | 11 | 0 | 0 | 11 |
| [.idea/inspectionProfiles/Project_Default.xml](/.idea/inspectionProfiles/Project_Default.xml) | XML | 28 | 0 | 0 | 28 |
| [.idea/modules.xml](/.idea/modules.xml) | XML | 8 | 0 | 0 | 8 |
| [.idea/sqldialects.xml](/.idea/sqldialects.xml) | XML | 6 | 0 | 0 | 6 |
| [.idea/vcs.xml](/.idea/vcs.xml) | XML | 6 | 0 | 0 | 6 |
| [Cargo.lock](/Cargo.lock) | TOML | 4,915 | 2 | 514 | 5,431 |
| [Cargo.toml](/Cargo.toml) | TOML | 54 | 1 | 4 | 59 |
| [app_config.toml](/app_config.toml) | TOML | 11 | 8 | 5 | 24 |
| [build.rs](/build.rs) | Rust | 35 | 0 | 3 | 38 |
| [forum-macros/Cargo.toml](/forum-macros/Cargo.toml) | TOML | 9 | 1 | 4 | 14 |
| [forum-macros/src/lib.rs](/forum-macros/src/lib.rs) | Rust | 30 | 0 | 8 | 38 |
| [src/config/app_config.rs](/src/config/app_config.rs) | Rust | 46 | 1 | 10 | 57 |
| [src/config/database.rs](/src/config/database.rs) | Rust | 56 | 17 | 12 | 85 |
| [src/config/meili.rs](/src/config/meili.rs) | Rust | 21 | 0 | 6 | 27 |
| [src/config/mod.rs](/src/config/mod.rs) | Rust | 13 | 0 | 3 | 16 |
| [src/config/permission.rs](/src/config/permission.rs) | Rust | 14 | 0 | 3 | 17 |
| [src/config/redis.rs](/src/config/redis.rs) | Rust | 36 | 0 | 9 | 45 |
| [src/config/s3.rs](/src/config/s3.rs) | Rust | 38 | 0 | 9 | 47 |
| [src/config/session.rs](/src/config/session.rs) | Rust | 61 | 0 | 7 | 68 |
| [src/dto/board.rs](/src/dto/board.rs) | Rust | 20 | 12 | 8 | 40 |
| [src/dto/course_tree.rs](/src/dto/course_tree.rs) | Rust | 25 | 0 | 5 | 30 |
| [src/dto/mod.rs](/src/dto/mod.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/entity/course.rs](/src/entity/course.rs) | Rust | 22 | 7 | 9 | 38 |
| [src/entity/homework.rs](/src/entity/homework.rs) | Rust | 29 | 11 | 13 | 53 |
| [src/entity/homework_uploaded.rs](/src/entity/homework_uploaded.rs) | Rust | 27 | 11 | 13 | 51 |
| [src/entity/log_login.rs](/src/entity/log_login.rs) | Rust | 18 | 7 | 9 | 34 |
| [src/entity/log_post.rs](/src/entity/log_post.rs) | Rust | 22 | 7 | 9 | 38 |
| [src/entity/mod.rs](/src/entity/mod.rs) | Rust | 10 | 0 | 1 | 11 |
| [src/entity/post.rs](/src/entity/post.rs) | Rust | 74 | 38 | 29 | 141 |
| [src/entity/student.rs](/src/entity/student.rs) | Rust | 38 | 19 | 22 | 79 |
| [src/entity/student_info.rs](/src/entity/student_info.rs) | Rust | 14 | 4 | 6 | 24 |
| [src/entity/tag.rs](/src/entity/tag.rs) | Rust | 19 | 5 | 7 | 31 |
| [src/entity/term.rs](/src/entity/term.rs) | Rust | 12 | 2 | 4 | 18 |
| [src/error/api_error.rs](/src/error/api_error.rs) | Rust | 39 | 0 | 6 | 45 |
| [src/error/auth_error.rs](/src/error/auth_error.rs) | Rust | 33 | 0 | 4 | 37 |
| [src/error/limit_error.rs](/src/error/limit_error.rs) | Rust | 17 | 0 | 4 | 21 |
| [src/error/mod.rs](/src/error/mod.rs) | Rust | 5 | 0 | 1 | 6 |
| [src/error/param_error.rs](/src/error/param_error.rs) | Rust | 22 | 0 | 5 | 27 |
| [src/error/proc_error.rs](/src/error/proc_error.rs) | Rust | 30 | 0 | 7 | 37 |
| [src/handler/auth_handler.rs](/src/handler/auth_handler.rs) | Rust | 122 | 2 | 16 | 140 |
| [src/handler/board_handler.rs](/src/handler/board_handler.rs) | Rust | 28 | 1 | 4 | 33 |
| [src/handler/course_handler.rs](/src/handler/course_handler.rs) | Rust | 72 | 12 | 7 | 91 |
| [src/handler/homework_handler.rs](/src/handler/homework_handler.rs) | Rust | 108 | 3 | 10 | 121 |
| [src/handler/metadata_handler.rs](/src/handler/metadata_handler.rs) | Rust | 20 | 1 | 3 | 24 |
| [src/handler/mod.rs](/src/handler/mod.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/handler/swagger_handler.rs](/src/handler/swagger_handler.rs) | Rust | 38 | 0 | 2 | 40 |
| [src/handler/user_handler.rs](/src/handler/user_handler.rs) | Rust | 37 | 0 | 6 | 43 |
| [src/main.rs](/src/main.rs) | Rust | 88 | 2 | 14 | 104 |
| [src/middleware/mod.rs](/src/middleware/mod.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/middleware/rate_limit.rs](/src/middleware/rate_limit.rs) | Rust | 45 | 0 | 4 | 49 |
| [src/repository/course_repo.rs](/src/repository/course_repo.rs) | Rust | 66 | 0 | 12 | 78 |
| [src/repository/homework_repo.rs](/src/repository/homework_repo.rs) | Rust | 70 | 0 | 12 | 82 |
| [src/repository/log_repo.rs](/src/repository/log_repo.rs) | Rust | 44 | 0 | 9 | 53 |
| [src/repository/mod.rs](/src/repository/mod.rs) | Rust | 5 | 0 | 1 | 6 |
| [src/repository/post_repo.rs](/src/repository/post_repo.rs) | Rust | 437 | 26 | 38 | 501 |
| [src/repository/user_repo.rs](/src/repository/user_repo.rs) | Rust | 52 | 0 | 10 | 62 |
| [src/response/api_response.rs](/src/response/api_response.rs) | Rust | 62 | 0 | 9 | 71 |
| [src/response/mod.rs](/src/response/mod.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/routes/auth_routes.rs](/src/routes/auth_routes.rs) | Rust | 17 | 0 | 2 | 19 |
| [src/routes/board_routes.rs](/src/routes/board_routes.rs) | Rust | 6 | 0 | 4 | 10 |
| [src/routes/course_routes.rs](/src/routes/course_routes.rs) | Rust | 10 | 0 | 4 | 14 |
| [src/routes/homework_routes.rs](/src/routes/homework_routes.rs) | Rust | 17 | 0 | 4 | 21 |
| [src/routes/metadata_routes.rs](/src/routes/metadata_routes.rs) | Rust | 6 | 0 | 4 | 10 |
| [src/routes/mod.rs](/src/routes/mod.rs) | Rust | 7 | 0 | 1 | 8 |
| [src/routes/root.rs](/src/routes/root.rs) | Rust | 75 | 0 | 10 | 85 |
| [src/routes/user_routes.rs](/src/routes/user_routes.rs) | Rust | 13 | 0 | 2 | 15 |
| [src/service/auth_service.rs](/src/service/auth_service.rs) | Rust | 98 | 0 | 17 | 115 |
| [src/service/board_service.rs](/src/service/board_service.rs) | Rust | 113 | 10 | 19 | 142 |
| [src/service/course_service.rs](/src/service/course_service.rs) | Rust | 280 | 15 | 40 | 335 |
| [src/service/homework_service.rs](/src/service/homework_service.rs) | Rust | 141 | 1 | 21 | 163 |
| [src/service/log_service.rs](/src/service/log_service.rs) | Rust | 55 | 0 | 13 | 68 |
| [src/service/metadata_service.rs](/src/service/metadata_service.rs) | Rust | 45 | 0 | 10 | 55 |
| [src/service/mod.rs](/src/service/mod.rs) | Rust | 9 | 0 | 1 | 10 |
| [src/service/post_service.rs](/src/service/post_service.rs) | Rust | 487 | 29 | 56 | 572 |
| [src/service/search_engine_service.rs](/src/service/search_engine_service.rs) | Rust | 22 | 2 | 7 | 31 |
| [src/service/user_service.rs](/src/service/user_service.rs) | Rust | 34 | 0 | 7 | 41 |
| [src/state/auth_state.rs](/src/state/auth_state.rs) | Rust | 14 | 0 | 3 | 17 |
| [src/state/board_state.rs](/src/state/board_state.rs) | Rust | 13 | 0 | 4 | 17 |
| [src/state/course_state.rs](/src/state/course_state.rs) | Rust | 16 | 0 | 4 | 20 |
| [src/state/homework_state.rs](/src/state/homework_state.rs) | Rust | 18 | 0 | 4 | 22 |
| [src/state/limit_state.rs](/src/state/limit_state.rs) | Rust | 13 | 0 | 3 | 16 |
| [src/state/metadata_state.rs](/src/state/metadata_state.rs) | Rust | 13 | 0 | 4 | 17 |
| [src/state/mod.rs](/src/state/mod.rs) | Rust | 7 | 0 | 1 | 8 |
| [src/state/user_state.rs](/src/state/user_state.rs) | Rust | 14 | 0 | 3 | 17 |
| [src/utils/mod.rs](/src/utils/mod.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/utils/string_utils.rs](/src/utils/string_utils.rs) | Rust | 13 | 0 | 3 | 16 |
| [templates/login.html](/templates/login.html) | HTML | 62 | 3 | 7 | 72 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)