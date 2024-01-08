/*
 Navicat Premium Data Transfer

 Source Server         : 本地
 Source Server Type    : MySQL
 Source Server Version : 50734 (5.7.34)
 Source Host           : localhost:3306
 Source Schema         : moyu_gateway

 Target Server Type    : MySQL
 Target Server Version : 50734 (5.7.34)
 File Encoding         : 65001

 Date: 08/01/2024 14:02:20
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for sys_services
-- ----------------------------
DROP TABLE IF EXISTS `sys_services`;
CREATE TABLE `sys_services` (
  `id` varchar(32) NOT NULL,
  `server_name` varchar(255) NOT NULL COMMENT '服务的名称，用于标识服务。',
  `url` varchar(255) NOT NULL COMMENT '服务的 URL 地址，存储服务的访问地址。',
  `description` text COMMENT '对服务的描述，可选字段，用于提供更多信息。',
  `protocol` varchar(255) NOT NULL COMMENT '服务的协议，例如 HTTP、HTTPS、TCP 等。',
  `port` int(11) DEFAULT NULL COMMENT '服务的端口号可选',
  `path` varchar(255) DEFAULT NULL COMMENT '服务的路径，用于指定服务的特定子路径。可选',
  `is_active` tinyint(4) NOT NULL DEFAULT '1' COMMENT '服务是否激活，默认为1 （1-启用，2-禁用）',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '修改时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of sys_services
-- ----------------------------
BEGIN;
INSERT INTO `sys_services` (`id`, `server_name`, `url`, `description`, `protocol`, `port`, `path`, `is_active`, `created_at`, `updated_at`) VALUES ('583929380946841604', 'fuwu1', 'baike.baidu.com', '', 'HTTPS', NULL, '/*', 1, '2023-12-31 16:06:53', '2024-01-08 05:44:50');
INSERT INTO `sys_services` (`id`, `server_name`, `url`, `description`, `protocol`, `port`, `path`, `is_active`, `created_at`, `updated_at`) VALUES ('584581110063697921', 'fuwu2', 'api.oioweb.cn', '', 'HTTPS', NULL, '/*', 1, '2024-01-02 11:16:38', '2024-01-08 05:33:08');
COMMIT;

SET FOREIGN_KEY_CHECKS = 1;
