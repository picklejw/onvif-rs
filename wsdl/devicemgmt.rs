﻿<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../../../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2018 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" targetNamespace="http://www.onvif.org/ver10/device/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver10/device/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" elementFormDefault="qualified" version="18.12">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>
			<!--===============================-->
			<xs:element name="GetServices">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IncludeCapability" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Indicates if the service capabilities (untyped) should be included in the response.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServicesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Service" type="tds:Service" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Each Service element contains information about one service.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Service">
				<xs:sequence>
					<xs:element name="Namespace" type="xs:anyURI">
						<xs:annotation>
							<xs:documentation>Namespace of the service being described. This parameter allows to match the service capabilities to the service. Note that only one set of capabilities is supported per namespace.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="XAddr" type="xs:anyURI">
						<xs:annotation>
							<xs:documentation>The transport addresses where the service can be reached. The scheme and IP part shall match the one used in the request (i.e. the GetServices request).</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Capabilities" minOccurs="0">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##any" processContents="lax">
									<xs:annotation>
										<xs:documentation>The placeholder for the service capabilities. The service capability element shall be returned here. For example for the device service that would be the tds:DeviceServiceCapabilities element (not complextype).</xs:documentation>
									</xs:annotation>
								</xs:any>
							</xs:sequence>
						</xs:complexType>
					</xs:element>
					<xs:element name="Version" type="tt:OnvifVersion">
						<xs:annotation>
							<xs:documentation>The version of the service (not the ONVIF core spec version).</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tds:DeviceServiceCapabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the device service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="DeviceServiceCapabilities">
				<xs:sequence>
					<xs:element name="Network" type="tds:NetworkCapabilities">
						<xs:annotation>
							<xs:documentation>Network capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Security" type="tds:SecurityCapabilities">
						<xs:annotation>
							<xs:documentation>Security capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="System" type="tds:SystemCapabilities">
						<xs:annotation>
							<xs:documentation>System capabilities.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Misc" type="tds:MiscCapabilities" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Capabilities that do not fit in any of the other categories.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<xs:element name="Capabilities" type="tds:DeviceServiceCapabilities"/>
			<!--===============================-->
			<xs:complexType name="NetworkCapabilities">
				<xs:attribute name="IPFilter" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for IP filtering.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="ZeroConfiguration" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for zeroconf.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="IPVersion6" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for IPv6.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DynDNS" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for dynamic DNS configuration.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Dot11Configuration" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for IEEE 802.11 configuration.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Dot1XConfigurations" type="xs:int">
					<xs:annotation>
						<xs:documentation>Indicates the maximum number of Dot1X configurations supported by the device</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HostnameFromDHCP" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for retrieval of hostname from DHCP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="NTP" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum number of NTP servers supported by the devices SetNTP command.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DHCPv6" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for Stateful IPv6 DHCP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="EAPMethodTypes">
				<xs:list itemType="xs:int"/>
			</xs:simpleType>
			<xs:complexType name="SecurityCapabilities">
				<xs:attribute name="TLS1.0" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for TLS 1.0.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="TLS1.1" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for TLS 1.1.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="TLS1.2" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for TLS 1.2.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="OnboardKeyGeneration" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for onboard key generation.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="AccessPolicyConfig" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for access policy configuration.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DefaultAccessPolicy" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for the ONVIF default access policy.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="Dot1X" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for IEEE 802.1X configuration.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RemoteUserHandling" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for remote user configuration. Used when accessing another device.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="X.509Token" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Security X.509 token.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SAMLToken" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Security SAML token.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="KerberosToken" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Security Kerberos token.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="UsernameToken" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Security Username token.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HttpDigest" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS over HTTP digest authenticated communication layer.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RELToken" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Security REL token.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SupportedEAPMethods" type="tds:EAPMethodTypes">
					<xs:annotation>
						<xs:documentation>EAP Methods supported by the device. The int values refer to the <a href="http://www.iana.org/assignments/eap-numbers/eap-numbers.xhtml">IANA EAP Registry</a>.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxUsers" type="xs:int">
					<xs:annotation>
						<xs:documentation>The maximum number of users that the device supports.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxUserNameLength" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum number of characters supported for the username by CreateUsers.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxPasswordLength" type="xs:int">
					<xs:annotation>
						<xs:documentation>Maximum number of characters supported for the password by CreateUsers and SetUser.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="SystemCapabilities">
				<xs:attribute name="DiscoveryResolve" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS Discovery resolve requests.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DiscoveryBye" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for WS-Discovery Bye.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RemoteDiscovery" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for remote discovery.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SystemBackup" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for system backup through MTOM.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SystemLogging" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for retrieval of system logging through MTOM.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="FirmwareUpgrade" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for firmware upgrade through MTOM.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HttpFirmwareUpgrade" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for firmware upgrade through HTTP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HttpSystemBackup" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for system backup through HTTP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HttpSystemLogging" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for retrieval of system logging through HTTP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="HttpSupportInformation" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for retrieving support information through HTTP.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="StorageConfiguration" type="xs:boolean">
					<xs:annotation>
						<xs:documentation>Indicates support for storage configuration interfaces.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="MaxStorageConfigurations" type="xs:int">
					<xs:annotation>
						<xs:documentation>Indicates maximum number of storage configurations supported.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="GeoLocationEntries" type="xs:int">
					<xs:annotation>
						<xs:documentation>If present signals support for geo location. The value signals the supported number of entries.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="AutoGeo" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation>List of supported automatic GeoLocation adjustment supported by the device. Valid items are defined by tds:AutoGeoMode.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="StorageTypesSupported" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation>Enumerates the supported StorageTypes, see tds:StorageType.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="AutoGeoModes">
				<xs:restriction base="xs:string">
					<xs:enumeration value="Location">
						<xs:annotation>	
							<xs:documentation>Automatic adjustment of the device location.</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="Heading">
						<xs:annotation>	
							<xs:documentation>Automatic adjustment of the device orientation relative to the compass also called yaw.</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="Leveling">
						<xs:annotation>	
							<xs:documentation>Automatic adjustment of the deviation from the horizon also called pitch and roll.</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
    			</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="MiscCapabilities">
				<xs:attribute name="AuxiliaryCommands" type="tt:StringAttrList">
					<xs:annotation>
						<xs:documentation>Lists of commands supported by SendAuxiliaryCommand.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:element name="GetDeviceInformation">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDeviceInformationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Manufacturer" type="xs:string">
							<xs:annotation>
								<xs:documentation>The manufactor of the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Model" type="xs:string">
							<xs:annotation>
								<xs:documentation>The device model.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="FirmwareVersion" type="xs:string">
							<xs:annotation>
								<xs:documentation>The firmware version in the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SerialNumber" type="xs:string">
							<xs:annotation>
								<xs:documentation>The serial number of the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="HardwareId" type="xs:string">
							<xs:annotation>
								<xs:documentation>The hardware ID of the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetSystemDateAndTime">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DateTimeType" type="tt:SetDateTimeType">
							<xs:annotation>
								<xs:documentation>Defines if the date and time is set via NTP or manually.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="DaylightSavings" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Automatically adjust Daylight savings if defined in TimeZone.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TimeZone" type="tt:TimeZone" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The time zone in POSIX 1003.1 format</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="UTCDateTime" type="tt:DateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Date and time in UTC. If time is obtained via NTP, UTCDateTime has no meaning</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetSystemDateAndTimeResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSystemDateAndTime">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSystemDateAndTimeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SystemDateAndTime" type="tt:SystemDateTime">
							<xs:annotation>
								<xs:documentation>Contains information whether system date and time are set manually or by NTP, daylight savings is on or off, time zone in POSIX 1003.1 format and system date and time in UTC and also local system date and time.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetSystemFactoryDefault">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="FactoryDefault" type="tt:FactoryDefaultType">
							<xs:annotation>
								<xs:documentation>Specifies the factory default action type.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetSystemFactoryDefaultResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="UpgradeSystemFirmware">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Firmware" type="tt:AttachmentData"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="UpgradeSystemFirmwareResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Message" type="xs:string" minOccurs="0" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SystemReboot">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="SystemRebootResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Message" type="xs:string">
							<xs:annotation>
								<xs:documentation>Contains the reboot message sent by the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RestoreSystem">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="BackupFiles" type="tt:BackupFile" minOccurs="1" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RestoreSystemResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSystemBackup">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSystemBackupResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="BackupFiles" type="tt:BackupFile" minOccurs="1" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSystemSupportInformation">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSystemSupportInformationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SupportInformation" type="tt:SupportInformation">
							<xs:annotation>
								<xs:documentation>Contains the arbitary device diagnostics information.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSystemLog">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="LogType" type="tt:SystemLogType">
							<xs:annotation>
								<xs:documentation>Specifies the type of system log to get.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSystemLogResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SystemLog" type="tt:SystemLog">
							<xs:annotation>
								<xs:documentation>Contains the system log information.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetScopes">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetScopesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Scopes" type="tt:Scope" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of URI definining the device scopes. Scope parameters can be of two types: fixed and configurable. Fixed parameters can not be altered.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetScopes">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Scopes" type="xs:anyURI" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of scope parameters that will replace all existing configurable scope parameters.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetScopesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddScopes">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ScopeItem" type="xs:anyURI" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of new configurable scope parameters that will be added to the existing configurable scope.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddScopesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveScopes">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ScopeItem" type="xs:anyURI" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of URIs that should be removed from the device scope.<br/>
									Note that the response message always will match the request or an error will be returned. The use of the response is for that reason deprecated.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveScopesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ScopeItem" type="xs:anyURI" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of URIs that has been removed from the device scope</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDiscoveryMode">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDiscoveryModeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DiscoveryMode" type="tt:DiscoveryMode">
							<xs:annotation>
								<xs:documentation>
									Indicator of discovery mode: Discoverable, NonDiscoverable.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDiscoveryMode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DiscoveryMode" type="tt:DiscoveryMode">
							<xs:annotation>
								<xs:documentation>
									Indicator of discovery mode: Discoverable, NonDiscoverable.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDiscoveryModeResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetRemoteDiscoveryMode">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRemoteDiscoveryModeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RemoteDiscoveryMode" type="tt:DiscoveryMode">
							<xs:annotation>
								<xs:documentation>
									Indicator of discovery mode: Discoverable, NonDiscoverable.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetRemoteDiscoveryMode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RemoteDiscoveryMode" type="tt:DiscoveryMode">
							<xs:annotation>
								<xs:documentation>
									Indicator of discovery mode: Discoverable, NonDiscoverable.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRemoteDiscoveryModeResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDPAddresses">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDPAddressesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DPAddress" type="tt:NetworkHost" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDPAddresses">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DPAddress" type="tt:NetworkHost" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDPAddressesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetEndpointReference">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetEndpointReferenceResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="GUID" type="xs:string"/>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetRemoteUser">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRemoteUserResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RemoteUser" type="tt:RemoteUser" minOccurs="0"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetRemoteUser">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RemoteUser" type="tt:RemoteUser" minOccurs="0"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRemoteUserResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetUsers">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetUsersResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="User" type="tt:User" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains a list of the onvif users and following information is included in each entry: username and user level.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateUsers">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="User" type="tt:User" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Creates new device users and corresponding credentials. Each user entry includes: username, password and user level. Either all users are created successfully or a fault message MUST be returned without creating any user. If trying to create several users with exactly the same username the request is rejected and no users are created. If password is missing, then fault message Too weak password is returned.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateUsersResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteUsers">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Username" type="xs:string" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Deletes users on an device and there may exist users that cannot be deleted to ensure access to the unit. Either all users are deleted successfully or a fault message MUST be returned and no users be deleted. If a username exists multiple times in the request, then a fault message is returned.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteUsersResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetUser">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="User" type="tt:User" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Updates the credentials for one or several users on an device. Either all change requests are processed successfully or a fault message MUST be returned. If the request contains the same username multiple times, a fault message is returned. </xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetUserResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetWsdlUrl">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetWsdlUrlResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="WsdlUrl" type="xs:anyURI"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCapabilities">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Category" type="tt:CapabilityCategory" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									List of categories to retrieve capability information on.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tt:Capabilities">
							<xs:annotation>
								<xs:documentation>
									Capability information.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetHostname">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetHostnameResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="HostnameInformation" type="tt:HostnameInformation">
							<xs:annotation>
								<xs:documentation>Contains the hostname information.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetHostname">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Name" type="xs:token">
							<xs:annotation>
								<xs:documentation>The hostname to set.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetHostnameResponse">
				<xs:complexType>
					<xs:sequence>
				    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetHostnameFromDHCP">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="FromDHCP" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>True if the hostname shall be obtained via DHCP.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetHostnameFromDHCPResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RebootNeeded" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>
									Indicates whether or not a reboot is required after configuration updates.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDNS">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDNSResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DNSInformation" type="tt:DNSInformation">
							<xs:annotation>
								<xs:documentation>
									DNS information.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDNS">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="FromDHCP" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>
									Indicate if the DNS address is to be retrieved using DHCP.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SearchDomain" type="xs:token" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									DNS search domain.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="DNSManual" type="tt:IPAddress" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									DNS address(es) set manually.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDNSResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNTP">
				<xs:complexType>
					<xs:sequence>
				    </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNTPResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NTPInformation" type="tt:NTPInformation">
							<xs:annotation>
								<xs:documentation>
									NTP information.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetNTP">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="FromDHCP" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>
									Indicate if NTP address information is to be retrieved using DHCP.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="NTPManual" type="tt:NetworkHost" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									Manual NTP settings.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetNTPResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDynamicDNS">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDynamicDNSResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DynamicDNSInformation" type="tt:DynamicDNSInformation">
							<xs:annotation>
								<xs:documentation>
									Dynamic DNS information.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDynamicDNS">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Type" type="tt:DynamicDNSType">
							<xs:annotation>
								<xs:documentation>
									Dynamic DNS type.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Name" type="tt:DNSName" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
									DNS name.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TTL" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
									DNS record time to live.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDynamicDNSResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNetworkInterfaces">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNetworkInterfacesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NetworkInterfaces" type="tt:NetworkInterface" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									List of network interfaces.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetNetworkInterfaces">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="InterfaceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>
									Symbolic network interface name.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="NetworkInterface" type="tt:NetworkInterfaceSetConfiguration">
							<xs:annotation>
								<xs:documentation>
									Network interface name.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetNetworkInterfacesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RebootNeeded" type="xs:boolean" minOccurs="1" maxOccurs="1">
							<xs:annotation>
								<xs:documentation>
									Indicates whether or not a reboot is required after configuration updates.
									If a device responds with RebootNeeded set to false, the device can be reached
									via the new IP address without further action. A client should be aware that a device
									may not be responsive for a short period of time until it signals availability at
									the new address via the discovery Hello messages.
									If a device responds with RebootNeeded set to true, it will be further available under
									its previous IP address. The settings will only be activated when the device is
									rebooted via the SystemReboot command.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNetworkProtocols">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNetworkProtocolsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NetworkProtocols" type="tt:NetworkProtocol" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Contains an array of defined protocols supported by the device. There are three protocols defined; HTTP, HTTPS and RTSP. The following parameters can be retrieved for each protocol: port and enable/disable.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetNetworkProtocols">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NetworkProtocols" type="tt:NetworkProtocol" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Configures one or more defined network protocols supported by the device. There are currently three protocols defined; HTTP, HTTPS and RTSP. The following parameters can be set for each protocol: port and enable/disable.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetNetworkProtocolsResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetNetworkDefaultGateway">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetNetworkDefaultGatewayResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NetworkGateway" type="tt:NetworkGateway">
							<xs:annotation>
								<xs:documentation>Gets the default IPv4 and IPv6 gateway settings from the device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetNetworkDefaultGateway">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IPv4Address" type="tt:IPv4Address" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Sets IPv4 gateway address used as default setting.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="IPv6Address" type="tt:IPv6Address" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>Sets IPv6 gateway address used as default setting.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetNetworkDefaultGatewayResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetZeroConfiguration">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetZeroConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ZeroConfiguration" type="tt:NetworkZeroConfiguration">
							<xs:annotation>
								<xs:documentation>Contains the zero-configuration.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetZeroConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="InterfaceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Unique identifier referencing the physical interface.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Enabled" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Specifies if the zero-configuration should be enabled or not.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetZeroConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetIPAddressFilter">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetIPAddressFilterResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IPAddressFilter" type="tt:IPAddressFilter"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetIPAddressFilter">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IPAddressFilter" type="tt:IPAddressFilter"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetIPAddressFilterResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="AddIPAddressFilter">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IPAddressFilter" type="tt:IPAddressFilter"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="AddIPAddressFilterResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="RemoveIPAddressFilter">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="IPAddressFilter" type="tt:IPAddressFilter"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="RemoveIPAddressFilterResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAccessPolicy">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAccessPolicyResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PolicyFile" type="tt:BinaryData"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAccessPolicy">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="PolicyFile" type="tt:BinaryData"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAccessPolicyResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateCertificate">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="xs:token" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Certificate id.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Subject" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Identification of the entity associated with the public-key.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ValidNotBefore" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Certificate validity start date.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="ValidNotAfter" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Certificate expiry start date.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateCertificateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NvtCertificate" type="tt:Certificate">
							<xs:annotation>
								<xs:documentation>
									base64 encoded DER representation of certificate.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertificates">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertificatesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NvtCertificate" type="tt:Certificate" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									Id and base64 encoded DER representation of all available certificates.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertificatesStatus">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertificatesStatusResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateStatus" type="tt:CertificateStatus" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									Indicates if a certificate is used in an optional HTTPS configuration of the device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetCertificatesStatus">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateStatus" type="tt:CertificateStatus" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									Indicates if a certificate is to be used in an optional HTTPS configuration of the device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetCertificatesStatusResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteCertificates">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="xs:token" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									List of ids of certificates to delete.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteCertificatesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPkcs10Request">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="xs:token">
							<xs:annotation>
								<xs:documentation>
									List of ids of certificates to delete.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Subject" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
									Relative Dinstinguished Name(RDN) CommonName(CN).
								</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Attributes" type="tt:BinaryData" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
									Optional base64 encoded DER attributes.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPkcs10RequestResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Pkcs10Request" type="tt:BinaryData">
							<xs:annotation>
								<xs:documentation>
									base64 encoded DER representation of certificate.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="LoadCertificates">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="NVTCertificate" type="tt:Certificate" minOccurs="1" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
									Optional id and base64 encoded DER representation of certificate.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="LoadCertificatesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetClientCertificateMode">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetClientCertificateModeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Enabled" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>
									Indicates whether or not client certificates are required by device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetClientCertificateMode">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Enabled" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>
									Indicates whether or not client certificates are required by device.
								</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetClientCertificateModeResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCACertificates">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCACertificatesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CACertificate" type="tt:Certificate" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="LoadCertificateWithPrivateKey">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateWithPrivateKey" type="tt:CertificateWithPrivateKey" minOccurs="1" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="LoadCertificateWithPrivateKeyResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetCertificateInformation">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateID" type="xs:token" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetCertificateInformationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CertificateInformation" type="tt:CertificateInformation" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="LoadCACertificates">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="CACertificate" type="tt:Certificate" minOccurs="1" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="LoadCACertificatesResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="CreateDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tt:Dot1XConfiguration" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="CreateDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tt:Dot1XConfiguration" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfigurationToken" type="tt:ReferenceToken" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tt:Dot1XConfiguration" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDot1XConfigurations">
				<xs:complexType>
					<xs:sequence>
			</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDot1XConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfiguration" type="tt:Dot1XConfiguration" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="DeleteDot1XConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Dot1XConfigurationToken" type="tt:ReferenceToken" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="DeleteDot1XConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetRelayOutputs">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRelayOutputsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutputs" type="tt:RelayOutput" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetRelayOutputSettings">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutputToken" type="tt:ReferenceToken"/>
						<xs:element name="Properties" type="tt:RelayOutputSettings"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRelayOutputSettingsResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetRelayOutputState">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutputToken" type="tt:ReferenceToken" minOccurs="1" maxOccurs="1"/>
						<xs:element name="LogicalState" type="tt:RelayLogicalState" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetRelayOutputStateResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SendAuxiliaryCommand">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AuxiliaryCommand" type="tt:AuxiliaryData" minOccurs="1" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SendAuxiliaryCommandResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AuxiliaryCommandResponse" type="tt:AuxiliaryData" minOccurs="0" maxOccurs="1"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDot11Capabilities">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDot11CapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tt:Dot11Capabilities"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetDot11Status">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="InterfaceToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDot11StatusResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Status" type="tt:Dot11Status"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="ScanAvailableDot11Networks">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="InterfaceToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="ScanAvailableDot11NetworksResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Networks" type="tt:Dot11AvailableNetworks" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSystemUris">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSystemUrisResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SystemLogUris" type="tt:SystemLogUriList" minOccurs="0" maxOccurs="1"/>
						<xs:element name="SupportInfoUri" type="xs:anyURI" minOccurs="0" maxOccurs="1"/>
						<xs:element name="SystemBackupUri" type="xs:anyURI" minOccurs="0" maxOccurs="1"/>
						<xs:element name="Extension" minOccurs="0">
							<xs:complexType>
								<xs:sequence>
									<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
								</xs:sequence>
							</xs:complexType>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="StartFirmwareUpgrade">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="StartFirmwareUpgradeResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="UploadUri" type="xs:anyURI"/>
						<xs:element name="UploadDelay" type="xs:duration"/>
						<xs:element name="ExpectedDownTime" type="xs:duration"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="StartSystemRestore">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="StartSystemRestoreResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="UploadUri" type="xs:anyURI"/>
						<xs:element name="ExpectedDownTime" type="xs:duration"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			
			<xs:complexType name="UserCredential">
				<xs:sequence>
					<xs:element name="UserName" type="xs:string">
						<xs:annotation>	
							<xs:documentation>User name</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Password" type="xs:string" minOccurs="0">
						<xs:annotation>	
							<xs:documentation> optional password</xs:documentation> 
						</xs:annotation>
					</xs:element>
					<xs:element name="Extension" minOccurs="0">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			
			<xs:simpleType name="StorageType">
				<xs:restriction base="xs:string">
					<xs:enumeration value="NFS">
						<xs:annotation>	
							<xs:documentation>NFS protocol</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="CIFS">
						<xs:annotation>
							<xs:documentation>CIFS protocol</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="CDMI">
						<xs:annotation>
							<xs:documentation>CDMI protocol</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
					<xs:enumeration value="FTP">
						<xs:annotation>
							<xs:documentation>FTP protocol</xs:documentation>
						</xs:annotation>
					</xs:enumeration>
				</xs:restriction>
			</xs:simpleType>
			
			<xs:complexType name="StorageConfigurationData">
				<xs:sequence>
					<xs:element name="LocalPath" type="xs:anyURI" minOccurs="0">
						<xs:annotation>
			  				<xs:documentation> local path </xs:documentation>
			  			</xs:annotation>
					</xs:element>
					<xs:element name="StorageUri" type="xs:anyURI" minOccurs="0">
			  			<xs:annotation>
			  				<xs:documentation> Storage server address </xs:documentation>
			  			</xs:annotation>
			  		</xs:element>
					<xs:element name="User" type="tds:UserCredential"  minOccurs="0">
			  			<xs:annotation>
			  				<xs:documentation> User credential for the storage server </xs:documentation>
			  			</xs:annotation>
			  		</xs:element>
					<xs:element name="Extension" minOccurs="0">
						<xs:complexType>
							<xs:sequence>
								<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
							</xs:sequence>
						</xs:complexType>
					</xs:element>
				</xs:sequence>
				<xs:attribute name="type" type="xs:string" use="required">
					<xs:annotation>
						<xs:documentation>StorageType lists the acceptable values for type attribute </xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			 </xs:complexType>

			<xs:complexType name="StorageConfiguration">
				<xs:complexContent>
					<xs:extension base="tt:DeviceEntity">
						<xs:sequence>
							<xs:element name="Data" type="tds:StorageConfigurationData">
							</xs:element>
						</xs:sequence>
					</xs:extension>
				</xs:complexContent>
			</xs:complexType>

			<!--===============================-->
			<!--===============================-->
			<xs:element name="GetStorageConfigurations">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetStorageConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StorageConfigurations" type="tds:StorageConfiguration" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<!--===============================-->
			<!--===============================-->
 			<xs:element name="CreateStorageConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StorageConfiguration" type="tds:StorageConfigurationData">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="CreateStorageConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<!--===============================-->
 			<xs:element name="GetStorageConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="GetStorageConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StorageConfiguration" type="tds:StorageConfiguration">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<!--===============================-->
 			<xs:element name="SetStorageConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StorageConfiguration" type="tds:StorageConfiguration">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="SetStorageConfigurationResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<!--===============================-->
 			<xs:element name="DeleteStorageConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken">
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="DeleteStorageConfigurationResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<xs:element name="GetGeoLocation">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="GetGeoLocationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Location" type="tt:LocationEntity" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<xs:element name="SetGeoLocation">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Location" type="tt:LocationEntity" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="SetGeoLocationResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<xs:element name="DeleteGeoLocation">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Location" type="tt:LocationEntity" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>			
			<xs:element name="DeleteGeoLocationResponse">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>	 		
			<!--===============================-->
			<!--===============================-->
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServicesRequest">
		<wsdl:part name="parameters" element="tds:GetServices"/>
	</wsdl:message>
	<wsdl:message name="GetServicesResponse">
		<wsdl:part name="parameters" element="tds:GetServicesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tds:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tds:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDeviceInformationRequest">
		<wsdl:part name="parameters" element="tds:GetDeviceInformation"/>
	</wsdl:message>
	<wsdl:message name="GetDeviceInformationResponse">
		<wsdl:part name="parameters" element="tds:GetDeviceInformationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetSystemDateAndTimeRequest">
		<wsdl:part name="parameters" element="tds:SetSystemDateAndTime"/>
	</wsdl:message>
	<wsdl:message name="SetSystemDateAndTimeResponse">
		<wsdl:part name="parameters" element="tds:SetSystemDateAndTimeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSystemDateAndTimeRequest">
		<wsdl:part name="parameters" element="tds:GetSystemDateAndTime"/>
	</wsdl:message>
	<wsdl:message name="GetSystemDateAndTimeResponse">
		<wsdl:part name="parameters" element="tds:GetSystemDateAndTimeResponse"/>
	</wsdl:message>
	<wsdl:message name="SetSystemFactoryDefaultRequest">
		<wsdl:part name="parameters" element="tds:SetSystemFactoryDefault"/>
	</wsdl:message>
	<wsdl:message name="SetSystemFactoryDefaultResponse">
		<wsdl:part name="parameters" element="tds:SetSystemFactoryDefaultResponse"/>
	</wsdl:message>
	<wsdl:message name="UpgradeSystemFirmwareRequest">
		<wsdl:part name="parameters" element="tds:UpgradeSystemFirmware"/>
	</wsdl:message>
	<wsdl:message name="UpgradeSystemFirmwareResponse">
		<wsdl:part name="parameters" element="tds:UpgradeSystemFirmwareResponse"/>
	</wsdl:message>
	<wsdl:message name="SystemRebootRequest">
		<wsdl:part name="parameters" element="tds:SystemReboot"/>
	</wsdl:message>
	<wsdl:message name="SystemRebootResponse">
		<wsdl:part name="parameters" element="tds:SystemRebootResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSystemBackupRequest">
		<wsdl:part name="parameters" element="tds:GetSystemBackup"/>
	</wsdl:message>
	<wsdl:message name="GetSystemBackupResponse">
		<wsdl:part name="parameters" element="tds:GetSystemBackupResponse"/>
	</wsdl:message>
	<wsdl:message name="RestoreSystemRequest">
		<wsdl:part name="parameters" element="tds:RestoreSystem"/>
	</wsdl:message>
	<wsdl:message name="RestoreSystemResponse">
		<wsdl:part name="parameters" element="tds:RestoreSystemResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSystemSupportInformationRequest">
		<wsdl:part name="parameters" element="tds:GetSystemSupportInformation"/>
	</wsdl:message>
	<wsdl:message name="GetSystemSupportInformationResponse">
		<wsdl:part name="parameters" element="tds:GetSystemSupportInformationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSystemLogRequest">
		<wsdl:part name="parameters" element="tds:GetSystemLog"/>
	</wsdl:message>
	<wsdl:message name="GetSystemLogResponse">
		<wsdl:part name="parameters" element="tds:GetSystemLogResponse"/>
	</wsdl:message>
	<wsdl:message name="GetScopesRequest">
		<wsdl:part name="parameters" element="tds:GetScopes"/>
	</wsdl:message>
	<wsdl:message name="GetScopesResponse">
		<wsdl:part name="parameters" element="tds:GetScopesResponse"/>
	</wsdl:message>
	<wsdl:message name="SetScopesRequest">
		<wsdl:part name="parameters" element="tds:SetScopes"/>
	</wsdl:message>
	<wsdl:message name="SetScopesResponse">
		<wsdl:part name="parameters" element="tds:SetScopesResponse"/>
	</wsdl:message>
	<wsdl:message name="AddScopesRequest">
		<wsdl:part name="parameters" element="tds:AddScopes"/>
	</wsdl:message>
	<wsdl:message name="AddScopesResponse">
		<wsdl:part name="parameters" element="tds:AddScopesResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveScopesRequest">
		<wsdl:part name="parameters" element="tds:RemoveScopes"/>
	</wsdl:message>
	<wsdl:message name="RemoveScopesResponse">
		<wsdl:part name="parameters" element="tds:RemoveScopesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDiscoveryModeRequest">
		<wsdl:part name="parameters" element="tds:GetDiscoveryMode"/>
	</wsdl:message>
	<wsdl:message name="GetDiscoveryModeResponse">
		<wsdl:part name="parameters" element="tds:GetDiscoveryModeResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDiscoveryModeRequest">
		<wsdl:part name="parameters" element="tds:SetDiscoveryMode"/>
	</wsdl:message>
	<wsdl:message name="SetDiscoveryModeResponse">
		<wsdl:part name="parameters" element="tds:SetDiscoveryModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRemoteDiscoveryModeRequest">
		<wsdl:part name="parameters" element="tds:GetRemoteDiscoveryMode"/>
	</wsdl:message>
	<wsdl:message name="GetRemoteDiscoveryModeResponse">
		<wsdl:part name="parameters" element="tds:GetRemoteDiscoveryModeResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRemoteDiscoveryModeRequest">
		<wsdl:part name="parameters" element="tds:SetRemoteDiscoveryMode"/>
	</wsdl:message>
	<wsdl:message name="SetRemoteDiscoveryModeResponse">
		<wsdl:part name="parameters" element="tds:SetRemoteDiscoveryModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDPAddressesRequest">
		<wsdl:part name="parameters" element="tds:GetDPAddresses"/>
	</wsdl:message>
	<wsdl:message name="GetDPAddressesResponse">
		<wsdl:part name="parameters" element="tds:GetDPAddressesResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDPAddressesRequest">
		<wsdl:part name="parameters" element="tds:SetDPAddresses"/>
	</wsdl:message>
	<wsdl:message name="SetDPAddressesResponse">
		<wsdl:part name="parameters" element="tds:SetDPAddressesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetEndpointReferenceRequest">
		<wsdl:part name="parameters" element="tds:GetEndpointReference"/>
	</wsdl:message>
	<wsdl:message name="GetEndpointReferenceResponse">
		<wsdl:part name="parameters" element="tds:GetEndpointReferenceResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRemoteUserRequest">
		<wsdl:part name="parameters" element="tds:GetRemoteUser"/>
	</wsdl:message>
	<wsdl:message name="GetRemoteUserResponse">
		<wsdl:part name="parameters" element="tds:GetRemoteUserResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRemoteUserRequest">
		<wsdl:part name="parameters" element="tds:SetRemoteUser"/>
	</wsdl:message>
	<wsdl:message name="SetRemoteUserResponse">
		<wsdl:part name="parameters" element="tds:SetRemoteUserResponse"/>
	</wsdl:message>
	<wsdl:message name="GetUsersRequest">
		<wsdl:part name="parameters" element="tds:GetUsers"/>
	</wsdl:message>
	<wsdl:message name="GetUsersResponse">
		<wsdl:part name="parameters" element="tds:GetUsersResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateUsersRequest">
		<wsdl:part name="parameters" element="tds:CreateUsers"/>
	</wsdl:message>
	<wsdl:message name="CreateUsersResponse">
		<wsdl:part name="parameters" element="tds:CreateUsersResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteUsersRequest">
		<wsdl:part name="parameters" element="tds:DeleteUsers"/>
	</wsdl:message>
	<wsdl:message name="DeleteUsersResponse">
		<wsdl:part name="parameters" element="tds:DeleteUsersResponse"/>
	</wsdl:message>
	<wsdl:message name="SetUserRequest">
		<wsdl:part name="parameters" element="tds:SetUser"/>
	</wsdl:message>
	<wsdl:message name="SetUserResponse">
		<wsdl:part name="parameters" element="tds:SetUserResponse"/>
	</wsdl:message>
	<wsdl:message name="GetWsdlUrlRequest">
		<wsdl:part name="parameters" element="tds:GetWsdlUrl"/>
	</wsdl:message>
	<wsdl:message name="GetWsdlUrlResponse">
		<wsdl:part name="parameters" element="tds:GetWsdlUrlResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCapabilitiesRequest">
		<wsdl:part name="parameters" element="tds:GetCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetCapabilitiesResponse">
		<wsdl:part name="parameters" element="tds:GetCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetHostnameRequest">
		<wsdl:part name="parameters" element="tds:GetHostname"/>
	</wsdl:message>
	<wsdl:message name="GetHostnameResponse">
		<wsdl:part name="parameters" element="tds:GetHostnameResponse"/>
	</wsdl:message>
	<wsdl:message name="SetHostnameRequest">
		<wsdl:part name="parameters" element="tds:SetHostname"/>
	</wsdl:message>
	<wsdl:message name="SetHostnameResponse">
		<wsdl:part name="parameters" element="tds:SetHostnameResponse"/>
	</wsdl:message>
	<wsdl:message name="SetHostnameFromDHCPRequest">
		<wsdl:part name="parameters" element="tds:SetHostnameFromDHCP"/>
	</wsdl:message>
	<wsdl:message name="SetHostnameFromDHCPResponse">
		<wsdl:part name="parameters" element="tds:SetHostnameFromDHCPResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDNSRequest">
		<wsdl:part name="parameters" element="tds:GetDNS"/>
	</wsdl:message>
	<wsdl:message name="GetDNSResponse">
		<wsdl:part name="parameters" element="tds:GetDNSResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDNSRequest">
		<wsdl:part name="parameters" element="tds:SetDNS"/>
	</wsdl:message>
	<wsdl:message name="SetDNSResponse">
		<wsdl:part name="parameters" element="tds:SetDNSResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNTPRequest">
		<wsdl:part name="parameters" element="tds:GetNTP"/>
	</wsdl:message>
	<wsdl:message name="GetNTPResponse">
		<wsdl:part name="parameters" element="tds:GetNTPResponse"/>
	</wsdl:message>
	<wsdl:message name="SetNTPRequest">
		<wsdl:part name="parameters" element="tds:SetNTP"/>
	</wsdl:message>
	<wsdl:message name="SetNTPResponse">
		<wsdl:part name="parameters" element="tds:SetNTPResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDynamicDNSRequest">
		<wsdl:part name="parameters" element="tds:GetDynamicDNS"/>
	</wsdl:message>
	<wsdl:message name="GetDynamicDNSResponse">
		<wsdl:part name="parameters" element="tds:GetDynamicDNSResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDynamicDNSRequest">
		<wsdl:part name="parameters" element="tds:SetDynamicDNS"/>
	</wsdl:message>
	<wsdl:message name="SetDynamicDNSResponse">
		<wsdl:part name="parameters" element="tds:SetDynamicDNSResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkInterfacesRequest">
		<wsdl:part name="parameters" element="tds:GetNetworkInterfaces"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkInterfacesResponse">
		<wsdl:part name="parameters" element="tds:GetNetworkInterfacesResponse"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkInterfacesRequest">
		<wsdl:part name="parameters" element="tds:SetNetworkInterfaces"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkInterfacesResponse">
		<wsdl:part name="parameters" element="tds:SetNetworkInterfacesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkProtocolsRequest">
		<wsdl:part name="parameters" element="tds:GetNetworkProtocols"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkProtocolsResponse">
		<wsdl:part name="parameters" element="tds:GetNetworkProtocolsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkProtocolsRequest">
		<wsdl:part name="parameters" element="tds:SetNetworkProtocols"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkProtocolsResponse">
		<wsdl:part name="parameters" element="tds:SetNetworkProtocolsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkDefaultGatewayRequest">
		<wsdl:part name="parameters" element="tds:GetNetworkDefaultGateway"/>
	</wsdl:message>
	<wsdl:message name="GetNetworkDefaultGatewayResponse">
		<wsdl:part name="parameters" element="tds:GetNetworkDefaultGatewayResponse"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkDefaultGatewayRequest">
		<wsdl:part name="parameters" element="tds:SetNetworkDefaultGateway"/>
	</wsdl:message>
	<wsdl:message name="SetNetworkDefaultGatewayResponse">
		<wsdl:part name="parameters" element="tds:SetNetworkDefaultGatewayResponse"/>
	</wsdl:message>
	<wsdl:message name="GetZeroConfigurationRequest">
		<wsdl:part name="parameters" element="tds:GetZeroConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetZeroConfigurationResponse">
		<wsdl:part name="parameters" element="tds:GetZeroConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetZeroConfigurationRequest">
		<wsdl:part name="parameters" element="tds:SetZeroConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetZeroConfigurationResponse">
		<wsdl:part name="parameters" element="tds:SetZeroConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetIPAddressFilterRequest">
		<wsdl:part name="parameters" element="tds:GetIPAddressFilter"/>
	</wsdl:message>
	<wsdl:message name="GetIPAddressFilterResponse">
		<wsdl:part name="parameters" element="tds:GetIPAddressFilterResponse"/>
	</wsdl:message>
	<wsdl:message name="SetIPAddressFilterRequest">
		<wsdl:part name="parameters" element="tds:SetIPAddressFilter"/>
	</wsdl:message>
	<wsdl:message name="SetIPAddressFilterResponse">
		<wsdl:part name="parameters" element="tds:SetIPAddressFilterResponse"/>
	</wsdl:message>
	<wsdl:message name="AddIPAddressFilterRequest">
		<wsdl:part name="parameters" element="tds:AddIPAddressFilter"/>
	</wsdl:message>
	<wsdl:message name="AddIPAddressFilterResponse">
		<wsdl:part name="parameters" element="tds:AddIPAddressFilterResponse"/>
	</wsdl:message>
	<wsdl:message name="RemoveIPAddressFilterRequest">
		<wsdl:part name="parameters" element="tds:RemoveIPAddressFilter"/>
	</wsdl:message>
	<wsdl:message name="RemoveIPAddressFilterResponse">
		<wsdl:part name="parameters" element="tds:RemoveIPAddressFilterResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAccessPolicyRequest">
		<wsdl:part name="parameters" element="tds:GetAccessPolicy"/>
	</wsdl:message>
	<wsdl:message name="GetAccessPolicyResponse">
		<wsdl:part name="parameters" element="tds:GetAccessPolicyResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAccessPolicyRequest">
		<wsdl:part name="parameters" element="tds:SetAccessPolicy"/>
	</wsdl:message>
	<wsdl:message name="SetAccessPolicyResponse">
		<wsdl:part name="parameters" element="tds:SetAccessPolicyResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateCertificateRequest">
		<wsdl:part name="parameters" element="tds:CreateCertificate"/>
	</wsdl:message>
	<wsdl:message name="CreateCertificateResponse">
		<wsdl:part name="parameters" element="tds:CreateCertificateResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertificatesRequest">
		<wsdl:part name="parameters" element="tds:GetCertificates"/>
	</wsdl:message>
	<wsdl:message name="GetCertificatesResponse">
		<wsdl:part name="parameters" element="tds:GetCertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertificatesStatusRequest">
		<wsdl:part name="parameters" element="tds:GetCertificatesStatus"/>
	</wsdl:message>
	<wsdl:message name="GetCertificatesStatusResponse">
		<wsdl:part name="parameters" element="tds:GetCertificatesStatusResponse"/>
	</wsdl:message>
	<wsdl:message name="SetCertificatesStatusRequest">
		<wsdl:part name="parameters" element="tds:SetCertificatesStatus"/>
	</wsdl:message>
	<wsdl:message name="SetCertificatesStatusResponse">
		<wsdl:part name="parameters" element="tds:SetCertificatesStatusResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificatesRequest">
		<wsdl:part name="parameters" element="tds:DeleteCertificates"/>
	</wsdl:message>
	<wsdl:message name="DeleteCertificatesResponse">
		<wsdl:part name="parameters" element="tds:DeleteCertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPkcs10RequestRequest">
		<wsdl:part name="parameters" element="tds:GetPkcs10Request"/>
	</wsdl:message>
	<wsdl:message name="GetPkcs10RequestResponse">
		<wsdl:part name="parameters" element="tds:GetPkcs10RequestResponse"/>
	</wsdl:message>
	<wsdl:message name="LoadCertificatesRequest">
		<wsdl:part name="parameters" element="tds:LoadCertificates"/>
	</wsdl:message>
	<wsdl:message name="LoadCertificatesResponse">
		<wsdl:part name="parameters" element="tds:LoadCertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetClientCertificateModeRequest">
		<wsdl:part name="parameters" element="tds:GetClientCertificateMode"/>
	</wsdl:message>
	<wsdl:message name="GetClientCertificateModeResponse">
		<wsdl:part name="parameters" element="tds:GetClientCertificateModeResponse"/>
	</wsdl:message>
	<wsdl:message name="SetClientCertificateModeRequest">
		<wsdl:part name="parameters" element="tds:SetClientCertificateMode"/>
	</wsdl:message>
	<wsdl:message name="SetClientCertificateModeResponse">
		<wsdl:part name="parameters" element="tds:SetClientCertificateModeResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputsRequest">
		<wsdl:part name="parameters" element="tds:GetRelayOutputs"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputsResponse">
		<wsdl:part name="parameters" element="tds:GetRelayOutputsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputSettingsRequest">
		<wsdl:part name="parameters" element="tds:SetRelayOutputSettings"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputSettingsResponse">
		<wsdl:part name="parameters" element="tds:SetRelayOutputSettingsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputStateRequest">
		<wsdl:part name="parameters" element="tds:SetRelayOutputState"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputStateResponse">
		<wsdl:part name="parameters" element="tds:SetRelayOutputStateResponse"/>
	</wsdl:message>
	<wsdl:message name="SendAuxiliaryCommandRequest">
		<wsdl:part name="parameters" element="tds:SendAuxiliaryCommand"/>
	</wsdl:message>
	<wsdl:message name="SendAuxiliaryCommandResponse">
		<wsdl:part name="parameters" element="tds:SendAuxiliaryCommandResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCACertificatesRequest">
		<wsdl:part name="parameters" element="tds:GetCACertificates"/>
	</wsdl:message>
	<wsdl:message name="GetCACertificatesResponse">
		<wsdl:part name="parameters" element="tds:GetCACertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="LoadCertificateWithPrivateKeyRequest">
		<wsdl:part name="parameters" element="tds:LoadCertificateWithPrivateKey"/>
	</wsdl:message>
	<wsdl:message name="LoadCertificateWithPrivateKeyResponse">
		<wsdl:part name="parameters" element="tds:LoadCertificateWithPrivateKeyResponse"/>
	</wsdl:message>
	<wsdl:message name="GetCertificateInformationRequest">
		<wsdl:part name="parameters" element="tds:GetCertificateInformation"/>
	</wsdl:message>
	<wsdl:message name="GetCertificateInformationResponse">
		<wsdl:part name="parameters" element="tds:GetCertificateInformationResponse"/>
	</wsdl:message>
	<wsdl:message name="LoadCACertificatesRequest">
		<wsdl:part name="parameters" element="tds:LoadCACertificates"/>
	</wsdl:message>
	<wsdl:message name="LoadCACertificatesResponse">
		<wsdl:part name="parameters" element="tds:LoadCACertificatesResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tds:CreateDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="CreateDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tds:CreateDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tds:SetDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tds:SetDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tds:GetDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tds:GetDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationsRequest">
		<wsdl:part name="parameters" element="tds:GetDot1XConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetDot1XConfigurationsResponse">
		<wsdl:part name="parameters" element="tds:GetDot1XConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteDot1XConfigurationRequest">
		<wsdl:part name="parameters" element="tds:DeleteDot1XConfiguration"/>
	</wsdl:message>
	<wsdl:message name="DeleteDot1XConfigurationResponse">
		<wsdl:part name="parameters" element="tds:DeleteDot1XConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDot11CapabilitiesRequest">
		<wsdl:part name="parameters" element="tds:GetDot11Capabilities"/>
	</wsdl:message>
	<wsdl:message name="GetDot11CapabilitiesResponse">
		<wsdl:part name="parameters" element="tds:GetDot11CapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDot11StatusRequest">
		<wsdl:part name="parameters" element="tds:GetDot11Status"/>
	</wsdl:message>
	<wsdl:message name="GetDot11StatusResponse">
		<wsdl:part name="parameters" element="tds:GetDot11StatusResponse"/>
	</wsdl:message>
	<wsdl:message name="ScanAvailableDot11NetworksRequest">
		<wsdl:part name="parameters" element="tds:ScanAvailableDot11Networks"/>
	</wsdl:message>
	<wsdl:message name="ScanAvailableDot11NetworksResponse">
		<wsdl:part name="parameters" element="tds:ScanAvailableDot11NetworksResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSystemUrisRequest">
		<wsdl:part name="parameters" element="tds:GetSystemUris"/>
	</wsdl:message>
	<wsdl:message name="GetSystemUrisResponse">
		<wsdl:part name="parameters" element="tds:GetSystemUrisResponse"/>
	</wsdl:message>
	<wsdl:message name="StartFirmwareUpgradeRequest">
		<wsdl:part name="parameters" element="tds:StartFirmwareUpgrade"/>
	</wsdl:message>
	<wsdl:message name="StartFirmwareUpgradeResponse">
		<wsdl:part name="parameters" element="tds:StartFirmwareUpgradeResponse"/>
	</wsdl:message>
	<wsdl:message name="StartSystemRestoreRequest">
		<wsdl:part name="parameters" element="tds:StartSystemRestore"/>
	</wsdl:message>
	<wsdl:message name="StartSystemRestoreResponse">
		<wsdl:part name="parameters" element="tds:StartSystemRestoreResponse"/>
	</wsdl:message>
	
	<wsdl:message name="GetStorageConfigurationsRequest">
		<wsdl:part name="parameters" element="tds:GetStorageConfigurations"/>
	</wsdl:message>
	<wsdl:message name="GetStorageConfigurationsResponse">
		<wsdl:part name="parameters" element="tds:GetStorageConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="CreateStorageConfigurationRequest">
		<wsdl:part name="parameters" element="tds:CreateStorageConfiguration"/>
	</wsdl:message>
	<wsdl:message name="CreateStorageConfigurationResponse">
		<wsdl:part name="parameters" element="tds:CreateStorageConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetStorageConfigurationRequest">
		<wsdl:part name="parameters" element="tds:GetStorageConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetStorageConfigurationResponse">
		<wsdl:part name="parameters" element="tds:GetStorageConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetStorageConfigurationRequest">
		<wsdl:part name="parameters" element="tds:SetStorageConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetStorageConfigurationResponse">
		<wsdl:part name="parameters" element="tds:SetStorageConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteStorageConfigurationRequest">
		<wsdl:part name="parameters" element="tds:DeleteStorageConfiguration"/>
	</wsdl:message>
	<wsdl:message name="DeleteStorageConfigurationResponse">
		<wsdl:part name="parameters" element="tds:DeleteStorageConfigurationResponse"/>
	</wsdl:message>	
	
	<wsdl:message name="GetGeoLocationRequest">
		<wsdl:part name="parameters" element="tds:GetGeoLocation"/>
	</wsdl:message>
	<wsdl:message name="GetGeoLocationResponse">
		<wsdl:part name="parameters" element="tds:GetGeoLocationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetGeoLocationRequest">
		<wsdl:part name="parameters" element="tds:SetGeoLocation"/>
	</wsdl:message>
	<wsdl:message name="SetGeoLocationResponse">
		<wsdl:part name="parameters" element="tds:SetGeoLocationResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationRequest">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocation"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationResponse">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocationResponse"/>
	</wsdl:message>	
	
	<wsdl:portType name="Device">
		<wsdl:operation name="GetServices">
			<wsdl:documentation>Returns information about services on the device.</wsdl:documentation>
			<wsdl:input message="tds:GetServicesRequest"/>
			<wsdl:output message="tds:GetServicesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the device service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tds:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tds:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDeviceInformation">
			<wsdl:documentation>This operation gets basic device information from the device.</wsdl:documentation>
			<wsdl:input message="tds:GetDeviceInformationRequest"/>
			<wsdl:output message="tds:GetDeviceInformationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetSystemDateAndTime">
			<wsdl:documentation>This operation sets the device system date and time. The device shall support the
				configuration of the daylight saving setting and of the manual system date and time (if
				applicable) or indication of NTP time (if applicable) through the SetSystemDateAndTime
				command. <br/>
				If system time and date are set manually, the client shall include UTCDateTime in the request.<br/>
				A TimeZone token which is not formed according to the rules of IEEE 1003.1 section 8.3 is considered as invalid timezone.<br/>
				The DayLightSavings flag should be set to true to activate any DST settings of the TimeZone string. 
				Clear the DayLightSavings flag if the DST portion of the TimeZone settings should be ignored.
			</wsdl:documentation>
			<wsdl:input message="tds:SetSystemDateAndTimeRequest"/>
			<wsdl:output message="tds:SetSystemDateAndTimeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSystemDateAndTime">
			<wsdl:documentation>This operation gets the device system date and time. The device shall support the return of
				the daylight saving setting and of the manual system date and time (if applicable) or indication
				of NTP time (if applicable) through the GetSystemDateAndTime command.<br/>
				A device shall provide the UTCDateTime information.</wsdl:documentation>
			<wsdl:input message="tds:GetSystemDateAndTimeRequest"/>
			<wsdl:output message="tds:GetSystemDateAndTimeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetSystemFactoryDefault">
			<wsdl:documentation>This operation reloads the parameters on the device to their factory default values.</wsdl:documentation>
			<wsdl:input message="tds:SetSystemFactoryDefaultRequest"/>
			<wsdl:output message="tds:SetSystemFactoryDefaultResponse"/>
		</wsdl:operation>
		<wsdl:operation name="UpgradeSystemFirmware">
			<wsdl:documentation>This operation upgrades a device firmware version. After a successful upgrade the response
				message is sent before the device reboots. The device should support firmware upgrade
				through the UpgradeSystemFirmware command. The exact format of the firmware data is
				outside the scope of this standard.</wsdl:documentation>
			<wsdl:input message="tds:UpgradeSystemFirmwareRequest"/>
			<wsdl:output message="tds:UpgradeSystemFirmwareResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SystemReboot">
			<wsdl:documentation>This operation reboots the device.</wsdl:documentation>
			<wsdl:input message="tds:SystemRebootRequest"/>
			<wsdl:output message="tds:SystemRebootResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RestoreSystem">
			<wsdl:documentation>This operation restores the system backup configuration files(s) previously retrieved from a
				device. The device should support restore of backup configuration file(s) through the
				RestoreSystem command. The exact format of the backup configuration file(s) is outside the
				scope of this standard. If the command is supported, it shall accept backup files returned by
				the GetSystemBackup command.</wsdl:documentation>
			<wsdl:input message="tds:RestoreSystemRequest"/>
			<wsdl:output message="tds:RestoreSystemResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSystemBackup">
			<wsdl:documentation>This operation is retrieves system backup configuration file(s) from a device. The device
				should support return of back up configuration file(s) through the GetSystemBackup command.
				The backup is returned with reference to a name and mime-type together with binary data.
				The exact format of the backup configuration files is outside the scope of this standard.</wsdl:documentation>
			<wsdl:input message="tds:GetSystemBackupRequest"/>
			<wsdl:output message="tds:GetSystemBackupResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSystemLog">
			<wsdl:documentation>This operation gets a system log from the device. The exact format of the system logs is outside the scope of this standard.</wsdl:documentation>
			<wsdl:input message="tds:GetSystemLogRequest"/>
			<wsdl:output message="tds:GetSystemLogResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSystemSupportInformation">
			<wsdl:documentation>This operation gets arbitary device diagnostics information from the device.</wsdl:documentation>
			<wsdl:input message="tds:GetSystemSupportInformationRequest"/>
			<wsdl:output message="tds:GetSystemSupportInformationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetScopes">
			<wsdl:documentation>This operation requests the scope parameters of a device. The scope parameters are used in
				the device discovery to match a probe message, see Section 7. The Scope parameters are of
				two different types: <ul>
					<li>Fixed</li>
					<li>Configurable</li>
				</ul>
				Fixed scope parameters are permanent device characteristics and cannot be removed through the device management interface. 
				The scope type is indicated in the scope list returned in the get scope parameters response. A device shall support 
				retrieval of discovery scope parameters through the GetScopes command. As some scope parameters are mandatory, 
				the device shall return a non-empty scope list in the response.</wsdl:documentation>
			<wsdl:input message="tds:GetScopesRequest"/>
			<wsdl:output message="tds:GetScopesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetScopes">
			<wsdl:documentation>This operation sets the scope parameters of a device. The scope parameters are used in the
				device discovery to match a probe message.
				This operation replaces all existing configurable scope parameters (not fixed parameters). If
				this shall be avoided, one should use the scope add command instead. The device shall
				support configuration of discovery scope parameters through the SetScopes command.</wsdl:documentation>
			<wsdl:input message="tds:SetScopesRequest"/>
			<wsdl:output message="tds:SetScopesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddScopes">
			<wsdl:documentation>This operation adds new configurable scope parameters to a device. The scope parameters
				are used in the device discovery to match a probe message. The device shall
				support addition of discovery scope parameters through the AddScopes command.</wsdl:documentation>
			<wsdl:input message="tds:AddScopesRequest"/>
			<wsdl:output message="tds:AddScopesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveScopes">
			<wsdl:documentation>This operation deletes scope-configurable scope parameters from a device. The scope
				parameters are used in the device discovery to match a probe message, see Section 7. The
				device shall support deletion of discovery scope parameters through the RemoveScopes
				command.
				Table</wsdl:documentation>
			<wsdl:input message="tds:RemoveScopesRequest"/>
			<wsdl:output message="tds:RemoveScopesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDiscoveryMode">
			<wsdl:documentation>This operation gets the discovery mode of a device. See Section 7.2 for the definition of the
				different device discovery modes. The device shall support retrieval of the discovery mode
				setting through the GetDiscoveryMode command.</wsdl:documentation>
			<wsdl:input message="tds:GetDiscoveryModeRequest"/>
			<wsdl:output message="tds:GetDiscoveryModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDiscoveryMode">
			<wsdl:documentation>This operation sets the discovery mode operation of a device. See Section 7.2 for the
				definition of the different device discovery modes. The device shall support configuration of
				the discovery mode setting through the SetDiscoveryMode command.</wsdl:documentation>
			<wsdl:input message="tds:SetDiscoveryModeRequest"/>
			<wsdl:output message="tds:SetDiscoveryModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRemoteDiscoveryMode">
			<wsdl:documentation>This operation gets the remote discovery mode of a device. See Section 7.4 for the definition
				of remote discovery extensions. A device that supports remote discovery shall support
				retrieval of the remote discovery mode setting through the GetRemoteDiscoveryMode
				command.</wsdl:documentation>
			<wsdl:input message="tds:GetRemoteDiscoveryModeRequest"/>
			<wsdl:output message="tds:GetRemoteDiscoveryModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRemoteDiscoveryMode">
			<wsdl:documentation>This operation sets the remote discovery mode of operation of a device. See Section 7.4 for
				the definition of remote discovery remote extensions. A device that supports remote discovery
				shall support configuration of the discovery mode setting through the
				SetRemoteDiscoveryMode command.</wsdl:documentation>
			<wsdl:input message="tds:SetRemoteDiscoveryModeRequest"/>
			<wsdl:output message="tds:SetRemoteDiscoveryModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDPAddresses">
			<wsdl:documentation>This operation gets the remote DP address or addresses from a device. If the device supports
				remote discovery, as specified in Section 7.4, the device shall support retrieval of the remote
				DP address(es) through the GetDPAddresses command.</wsdl:documentation>
			<wsdl:input message="tds:GetDPAddressesRequest"/>
			<wsdl:output message="tds:GetDPAddressesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDPAddresses">
			<wsdl:documentation>This operation sets the remote DP address or addresses on a device. If the device supports
				remote discovery, as specified in Section 7.4, the device shall support configuration of the
				remote DP address(es) through the SetDPAddresses command.</wsdl:documentation>
			<wsdl:input message="tds:SetDPAddressesRequest"/>
			<wsdl:output message="tds:SetDPAddressesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetEndpointReference">
			<wsdl:documentation>A client can ask for the device service endpoint reference address property that can be used
				to derive the password equivalent for remote user operation. The device shall support the
				GetEndpointReference command returning the address property of the device service
				endpoint reference.</wsdl:documentation>
			<wsdl:input message="tds:GetEndpointReferenceRequest"/>
			<wsdl:output message="tds:GetEndpointReferenceResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRemoteUser">
			<wsdl:documentation>This operation returns the configured remote user (if any). A device supporting remote user
				handling shall support this operation. The user is only valid for the WS-UserToken profile or
				as a HTTP / RTSP user.<br/>
				The algorithm to use for deriving the password is described in section 5.12.2.1 of the core specification.</wsdl:documentation>
			<wsdl:input message="tds:GetRemoteUserRequest"/>
			<wsdl:output message="tds:GetRemoteUserResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRemoteUser">
			<wsdl:documentation>This operation sets the remote user. A device supporting remote user handling shall support this
				operation. The user is only valid for the WS-UserToken profile or as a HTTP / RTSP user.<br/>
				The password that is set shall always be the original (not derived) password.<br/>
				If UseDerivedPassword is set password derivation shall be done by the device when connecting to a
				remote device.The algorithm to use for deriving the password is described in section 5.12.2.1 of the core specification.<br/>
				To remove the remote user SetRemoteUser should be called without the RemoteUser parameter.</wsdl:documentation>
			<wsdl:input message="tds:SetRemoteUserRequest"/>
			<wsdl:output message="tds:SetRemoteUserResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetUsers">
			<wsdl:documentation>This operation lists the registered users and corresponding credentials on a device. The
				device shall support retrieval of registered device users and their credentials for the user
				token through the GetUsers command.</wsdl:documentation>
			<wsdl:input message="tds:GetUsersRequest"/>
			<wsdl:output message="tds:GetUsersResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateUsers">
			<wsdl:documentation>This operation creates new device users and corresponding credentials on a device for authentication purposes. 
				The device shall support creation of device users and their credentials through the CreateUsers
				command. Either all users are created successfully or a fault message shall be returned
				without creating any user.<br/>
				ONVIF compliant devices are recommended to support password length of at least 28 bytes,
				as clients may follow the password derivation mechanism which results in 'password
				equivalent' of length 28 bytes, as described in section 3.1.2 of the ONVIF security white paper.</wsdl:documentation>
			<wsdl:input message="tds:CreateUsersRequest"/>
			<wsdl:output message="tds:CreateUsersResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteUsers">
			<wsdl:documentation>This operation deletes users on a device. The device shall support deletion of device users and their credentials 
				through the DeleteUsers command. A device may have one or more fixed users
				that cannot be deleted to ensure access to the unit. Either all users are deleted successfully or a
				fault message shall be returned and no users be deleted.</wsdl:documentation>
			<wsdl:input message="tds:DeleteUsersRequest"/>
			<wsdl:output message="tds:DeleteUsersResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetUser">
			<wsdl:documentation>This operation updates the settings for one or several users on a device for authentication purposes.
				The device shall support update of device users and their credentials through the SetUser command. 
				Either all change requests are processed successfully or a fault message shall be returned and no change requests be processed.</wsdl:documentation>
			<wsdl:input message="tds:SetUserRequest"/>
			<wsdl:output message="tds:SetUserResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetWsdlUrl">
			<wsdl:documentation>It is possible for an endpoint to request a URL that can be used to retrieve the complete
				schema and WSDL definitions of a device. The command gives in return a URL entry point
				where all the necessary product specific WSDL and schema definitions can be retrieved. The
				device shall provide a URL for WSDL and schema download through the GetWsdlUrl command.</wsdl:documentation>
			<wsdl:input message="tds:GetWsdlUrlRequest"/>
			<wsdl:output message="tds:GetWsdlUrlResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCapabilities">
			<wsdl:documentation>This method has been replaced by the more generic GetServices method.
			 For capabilities of individual services refer to the GetServiceCapabilities methods.</wsdl:documentation>
			<wsdl:input message="tds:GetCapabilitiesRequest"/>
			<wsdl:output message="tds:GetCapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetHostname">
			<wsdl:documentation>This operation is used by an endpoint to get the hostname from a device. The device shall
				return its hostname configurations through the GetHostname command.</wsdl:documentation>
			<wsdl:input message="tds:GetHostnameRequest"/>
			<wsdl:output message="tds:GetHostnameResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetHostname">
			<wsdl:documentation>This operation sets the hostname on a device. It shall be possible to set the device hostname
				configurations through the SetHostname command.<br/>
				A device shall accept string formated according to RFC 1123 section 2.1 or alternatively to RFC 952, 
				other string shall be considered as invalid strings. 
			</wsdl:documentation>
			<wsdl:input message="tds:SetHostnameRequest"/>
			<wsdl:output message="tds:SetHostnameResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetHostnameFromDHCP">
			<wsdl:documentation>This operation controls whether the hostname is set manually or retrieved via DHCP.</wsdl:documentation>
			<wsdl:input message="tds:SetHostnameFromDHCPRequest"/>
			<wsdl:output message="tds:SetHostnameFromDHCPResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDNS">
			<wsdl:documentation>This operation gets the DNS settings from a device. The device shall return its DNS
				configurations through the GetDNS command.</wsdl:documentation>
			<wsdl:input message="tds:GetDNSRequest"/>
			<wsdl:output message="tds:GetDNSResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDNS">
			<wsdl:documentation>This operation sets the DNS settings on a device. It shall be possible to set the device DNS
				configurations through the SetDNS command.</wsdl:documentation>
			<wsdl:input message="tds:SetDNSRequest"/>
			<wsdl:output message="tds:SetDNSResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNTP">
			<wsdl:documentation>This operation gets the NTP settings from a device. If the device supports NTP, it shall be
				possible to get the NTP server settings through the GetNTP command.</wsdl:documentation>
			<wsdl:input message="tds:GetNTPRequest"/>
			<wsdl:output message="tds:GetNTPResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetNTP">
			<wsdl:documentation>This operation sets the NTP settings on a device. If the device supports NTP, it shall be
				possible to set the NTP server settings through the SetNTP command.<br/>
				A device shall accept string formated according to RFC 1123 section 2.1 or alternatively to RFC 952, 
				other string shall be considered as invalid strings. <br/>
				Changes to the NTP server list will not affect the clock mode DateTimeType. Use SetSystemDateAndTime to activate NTP operation.
			</wsdl:documentation>
			<wsdl:input message="tds:SetNTPRequest"/>
			<wsdl:output message="tds:SetNTPResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDynamicDNS">
			<wsdl:documentation>This operation gets the dynamic DNS settings from a device. If the device supports dynamic
				DNS as specified in [RFC 2136] and [RFC 4702], it shall be possible to get the type, name
				and TTL through the GetDynamicDNS command.</wsdl:documentation>
			<wsdl:input message="tds:GetDynamicDNSRequest"/>
			<wsdl:output message="tds:GetDynamicDNSResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDynamicDNS">
			<wsdl:documentation>This operation sets the dynamic DNS settings on a device. If the device supports dynamic
				DNS as specified in [RFC 2136] and [RFC 4702], it shall be possible to set the type, name
				and TTL through the SetDynamicDNS command.</wsdl:documentation>
			<wsdl:input message="tds:SetDynamicDNSRequest"/>
			<wsdl:output message="tds:SetDynamicDNSResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkInterfaces">
			<wsdl:documentation>This operation gets the network interface configuration from a device. The device shall
				support return of network interface configuration settings as defined by the NetworkInterface
				type through the GetNetworkInterfaces command.</wsdl:documentation>
			<wsdl:input message="tds:GetNetworkInterfacesRequest"/>
			<wsdl:output message="tds:GetNetworkInterfacesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkInterfaces">
			<wsdl:documentation>This operation sets the network interface configuration on a device. The device shall support
				network configuration of supported network interfaces through the SetNetworkInterfaces
				command.<br/>
				For interoperability with a client unaware of the IEEE 802.11 extension a device shall retain
				its IEEE 802.11 configuration if the IEEE 802.11 configuration element isn’t present in the
				request.</wsdl:documentation>
			<wsdl:input message="tds:SetNetworkInterfacesRequest"/>
			<wsdl:output message="tds:SetNetworkInterfacesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkProtocols">
			<wsdl:documentation>This operation gets defined network protocols from a device. The device shall support the
				GetNetworkProtocols command returning configured network protocols.</wsdl:documentation>
			<wsdl:input message="tds:GetNetworkProtocolsRequest"/>
			<wsdl:output message="tds:GetNetworkProtocolsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkProtocols">
			<wsdl:documentation>This operation configures defined network protocols on a device. The device shall support
				configuration of defined network protocols through the SetNetworkProtocols command.</wsdl:documentation>
			<wsdl:input message="tds:SetNetworkProtocolsRequest"/>
			<wsdl:output message="tds:SetNetworkProtocolsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkDefaultGateway">
			<wsdl:documentation>This operation gets the default gateway settings from a device. The device shall support the
				GetNetworkDefaultGateway command returning configured default gateway address(es).</wsdl:documentation>
			<wsdl:input message="tds:GetNetworkDefaultGatewayRequest"/>
			<wsdl:output message="tds:GetNetworkDefaultGatewayResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkDefaultGateway">
			<wsdl:documentation>This operation sets the default gateway settings on a device. The device shall support
				configuration of default gateway through the SetNetworkDefaultGateway command.</wsdl:documentation>
			<wsdl:input message="tds:SetNetworkDefaultGatewayRequest"/>
			<wsdl:output message="tds:SetNetworkDefaultGatewayResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetZeroConfiguration">
			<wsdl:documentation>This operation gets the zero-configuration from a device. If the device supports dynamic IP
				configuration according to [RFC3927], it shall support the return of IPv4 zero configuration
				address and status through the GetZeroConfiguration command.<br/>
			Devices supporting zero configuration on more than one interface shall use the extension to list the additional interface settings.</wsdl:documentation>
			<wsdl:input message="tds:GetZeroConfigurationRequest"/>
			<wsdl:output message="tds:GetZeroConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetZeroConfiguration">
			<wsdl:documentation>This operation sets the zero-configuration. Use GetCapalities to get if zero-zero-configuration is supported or not.</wsdl:documentation>
			<wsdl:input message="tds:SetZeroConfigurationRequest"/>
			<wsdl:output message="tds:SetZeroConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetIPAddressFilter">
			<wsdl:documentation>This operation gets the IP address filter settings from a device. If the device supports device
				access control based on IP filtering rules (denied or accepted ranges of IP addresses), the
				device shall support the GetIPAddressFilter command.</wsdl:documentation>
			<wsdl:input message="tds:GetIPAddressFilterRequest"/>
			<wsdl:output message="tds:GetIPAddressFilterResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetIPAddressFilter">
			<wsdl:documentation>This operation sets the IP address filter settings on a device. If the device supports device
				access control based on IP filtering rules (denied or accepted ranges of IP addresses), the
				device shall support configuration of IP filtering rules through the SetIPAddressFilter
				command.</wsdl:documentation>
			<wsdl:input message="tds:SetIPAddressFilterRequest"/>
			<wsdl:output message="tds:SetIPAddressFilterResponse"/>
		</wsdl:operation>
		<wsdl:operation name="AddIPAddressFilter">
			<wsdl:documentation>This operation adds an IP filter address to a device. If the device supports device access
				control based on IP filtering rules (denied or accepted ranges of IP addresses), the device
				shall support adding of IP filtering addresses through the AddIPAddressFilter command.</wsdl:documentation>
			<wsdl:input message="tds:AddIPAddressFilterRequest"/>
			<wsdl:output message="tds:AddIPAddressFilterResponse"/>
		</wsdl:operation>
		<wsdl:operation name="RemoveIPAddressFilter">
			<wsdl:documentation>This operation deletes an IP filter address from a device. If the device supports device access
				control based on IP filtering rules (denied or accepted ranges of IP addresses), the device
				shall support deletion of IP filtering addresses through the RemoveIPAddressFilter command.</wsdl:documentation>
			<wsdl:input message="tds:RemoveIPAddressFilterRequest"/>
			<wsdl:output message="tds:RemoveIPAddressFilterResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAccessPolicy">
			<wsdl:documentation>Access to different services and sub-sets of services should be subject to access control. The
				WS-Security framework gives the prerequisite for end-point authentication. Authorization
				decisions can then be taken using an access security policy. This standard does not mandate
				any particular policy description format or security policy but this is up to the device
				manufacturer or system provider to choose policy and policy description format of choice.
				However, an access policy (in arbitrary format) can be requested using this command. If the
				device supports access policy settings based on WS-Security authentication, then the device
				shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:GetAccessPolicyRequest"/>
			<wsdl:output message="tds:GetAccessPolicyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAccessPolicy">
			<wsdl:documentation>This command sets the device access security policy (for more details on the access security
				policy see the Get command). If the device supports access policy settings
				based on WS-Security authentication, then the device shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:SetAccessPolicyRequest"/>
			<wsdl:output message="tds:SetAccessPolicyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateCertificate">
			<wsdl:documentation>This operation generates a private/public key pair and also can create a self-signed device
				certificate as a result of key pair generation. The certificate is created using a suitable
				onboard key pair generation mechanism.<br/>
				If a device supports onboard key pair generation, the device that supports TLS shall support
				this certificate creation command. And also, if a device supports onboard key pair generation,
				the device that support IEEE 802.1X shall support this command for the purpose of key pair
				generation. Certificates and key pairs are identified using certificate IDs. These IDs are either
				chosen by the certificate generation requester or by the device (in case that no ID value is
				given).</wsdl:documentation>
			<wsdl:input message="tds:CreateCertificateRequest"/>
			<wsdl:output message="tds:CreateCertificateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertificates">
			<wsdl:documentation>This operation gets all device server certificates (including self-signed) for the purpose of TLS
				authentication and all device client certificates for the purpose of IEEE 802.1X authentication.
				This command lists only the TLS server certificates and IEEE 802.1X client certificates for the
				device (neither trusted CA certificates nor trusted root certificates). The certificates are
				returned as binary data. A device that supports TLS shall support this command and the
				certificates shall be encoded using ASN.1 [X.681], [X.682], [X.683] DER [X.690] encoding
				rules.</wsdl:documentation>
			<wsdl:input message="tds:GetCertificatesRequest"/>
			<wsdl:output message="tds:GetCertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertificatesStatus">
			<wsdl:documentation>This operation is specific to TLS functionality. This operation gets the status
				(enabled/disabled) of the device TLS server certificates. A device that supports TLS shall
				support this command.</wsdl:documentation>
			<wsdl:input message="tds:GetCertificatesStatusRequest"/>
			<wsdl:output message="tds:GetCertificatesStatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetCertificatesStatus">
			<wsdl:documentation>This operation is specific to TLS functionality. This operation sets the status (enable/disable)
				of the device TLS server certificates. A device that supports TLS shall support this command.
				Typically only one device server certificate is allowed to be enabled at a time.</wsdl:documentation>
			<wsdl:input message="tds:SetCertificatesStatusRequest"/>
			<wsdl:output message="tds:SetCertificatesStatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificates">
			<wsdl:documentation>This operation deletes a certificate or multiple certificates. The device MAY also delete a
				private/public key pair which is coupled with the certificate to be deleted. The device that
				support either TLS or IEEE 802.1X shall support the deletion of a certificate or multiple
				certificates through this command. Either all certificates are deleted successfully or a fault
				message shall be returned without deleting any certificate.</wsdl:documentation>
			<wsdl:input message="tds:DeleteCertificatesRequest"/>
			<wsdl:output message="tds:DeleteCertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetPkcs10Request">
			<wsdl:documentation>This operation requests a PKCS #10 certificate signature request from the device. The
				returned information field shall be either formatted exactly as specified in [PKCS#10] or PEM
				encoded [PKCS#10] format. In order for this command to work, the device must already have
				a private/public key pair. This key pair should be referred by CertificateID as specified in the
				input parameter description. This CertificateID refers to the key pair generated using
				CreateCertificate command.<br/>
				A device that support onboard key pair generation that supports either TLS or IEEE 802.1X
				using client certificate shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:GetPkcs10RequestRequest"/>
			<wsdl:output message="tds:GetPkcs10RequestResponse"/>
		</wsdl:operation>
		<wsdl:operation name="LoadCertificates">
			<wsdl:documentation>TLS server certificate(s) or IEEE 802.1X client certificate(s) created using the PKCS#10
				certificate request command can be loaded into the device using this command (see Section
				8.4.13). The certificate ID in the request shall be present. The device may sort the received
				certificate(s) based on the public key and subject information in the certificate(s).
				The certificate ID in the request will be the ID value the client wish to have. The device is
				supposed to scan the generated key pairs present in the device to identify which is the
				correspondent key pair with the loaded certificate and then make the link between the
				certificate and the key pair.<br/>
				A device that supports onboard key pair generation that support either TLS or IEEE 802.1X
				shall support this command.<br/>
				The certificates shall be encoded using ASN.1 [X.681], [X.682], [X.683] DER [X.690] encoding
				rules.<br/>
				This command is applicable to any device type, although the parameter name is called for
				historical reasons NVTCertificate.</wsdl:documentation>
			<wsdl:input message="tds:LoadCertificatesRequest"/>
			<wsdl:output message="tds:LoadCertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetClientCertificateMode">
			<wsdl:documentation>This operation is specific to TLS functionality. This operation gets the status
				(enabled/disabled) of the device TLS client authentication. A device that supports TLS shall
				support this command.</wsdl:documentation>
			<wsdl:input message="tds:GetClientCertificateModeRequest"/>
			<wsdl:output message="tds:GetClientCertificateModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetClientCertificateMode">
			<wsdl:documentation>This operation is specific to TLS functionality. This operation sets the status
				(enabled/disabled) of the device TLS client authentication. A device that supports TLS shall
				support this command.</wsdl:documentation>
			<wsdl:input message="tds:SetClientCertificateModeRequest"/>
			<wsdl:output message="tds:SetClientCertificateModeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputs">
			<wsdl:documentation>This operation gets a list of all available relay outputs and their settings.<br/>
				This method has been depricated with version 2.0. Refer to the DeviceIO service.</wsdl:documentation>
			<wsdl:input message="tds:GetRelayOutputsRequest"/>
			<wsdl:output message="tds:GetRelayOutputsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputSettings">
			<wsdl:documentation>This operation sets the settings of a relay output.
				<br/>This method has been depricated with version 2.0. Refer to the DeviceIO service.</wsdl:documentation>
			<wsdl:input message="tds:SetRelayOutputSettingsRequest"/>
			<wsdl:output message="tds:SetRelayOutputSettingsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputState">
			<wsdl:documentation>This operation sets the state of a relay output.
				<br/>This method has been depricated with version 2.0. Refer to the DeviceIO service.</wsdl:documentation>
			<wsdl:input message="tds:SetRelayOutputStateRequest"/>
			<wsdl:output message="tds:SetRelayOutputStateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SendAuxiliaryCommand">
			<wsdl:documentation>Manage auxiliary commands supported by a device, such as controlling an Infrared (IR) lamp, 
				a heater or a wiper or a thermometer that is connected to the device.<br/>
				The supported commands can be retrieved via the AuxiliaryCommands capability.<br/>
				Although the name of the auxiliary commands can be freely defined, commands starting with the prefix tt: are 
				reserved to define frequently used commands and these reserved commands shall all share the "tt:command|parameter" syntax.
				<ul>
					<li>tt:Wiper|On – Request to start the wiper.</li>
					<li>tt:Wiper|Off – Request to stop the wiper.</li>
					<li>tt:Washer|On – Request to start the washer.</li>
					<li>tt:Washer|Off – Request to stop the washer.</li>
					<li>tt:WashingProcedure|On – Request to start the washing procedure.</li>
					<li>tt: WashingProcedure |Off – Request to stop the washing procedure.</li>
					<li>tt:IRLamp|On – Request to turn ON an IR illuminator attached to the unit.</li>
					<li>tt:IRLamp|Off – Request to turn OFF an IR illuminator attached to the unit.</li>
					<li>tt:IRLamp|Auto – Request to configure an IR illuminator attached to the unit so that it automatically turns ON and OFF.</li>
				</ul>
				A device that indicates auxiliary service capability shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:SendAuxiliaryCommandRequest"/>
			<wsdl:output message="tds:SendAuxiliaryCommandResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCACertificates">
			<wsdl:documentation>CA certificates will be loaded into a device and be used for the sake of following two cases.
				The one is for the purpose of TLS client authentication in TLS server function. The other one
				is for the purpose of Authentication Server authentication in IEEE 802.1X function. This
				operation gets all CA certificates loaded into a device. A device that supports either TLS client
				authentication or IEEE 802.1X shall support this command and the returned certificates shall
				be encoded using ASN.1 [X.681], [X.682], [X.683] DER [X.690] encoding rules.</wsdl:documentation>
			<wsdl:input message="tds:GetCACertificatesRequest"/>
			<wsdl:output message="tds:GetCACertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="LoadCertificateWithPrivateKey">
			<wsdl:documentation>There might be some cases that a Certificate Authority or some other equivalent creates a
				certificate without having PKCS#10 certificate signing request. In such cases, the certificate
				will be bundled in conjunction with its private key. This command will be used for such use
				case scenarios. The certificate ID in the request is optionally set to the ID value the client
				wish to have. If the certificate ID is not specified in the request, device can choose the ID
				accordingly.<br/>
				This operation imports a private/public key pair into the device.
				The certificates shall be encoded using ASN.1 [X.681], [X.682], [X.683] DER [X.690] encoding
				rules.<br/>
				A device that does not support onboard key pair generation and support either TLS or IEEE
				802.1X using client certificate shall support this command. A device that support onboard key
				pair generation MAY support this command. The security policy of a device that supports this
				operation should make sure that the private key is sufficiently protected.</wsdl:documentation>
			<wsdl:input message="tds:LoadCertificateWithPrivateKeyRequest"/>
			<wsdl:output message="tds:LoadCertificateWithPrivateKeyResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetCertificateInformation">
			<wsdl:documentation>This operation requests the information of a certificate specified by certificate ID. The device
				should respond with its “Issuer DN”, “Subject DN”, “Key usage”, "Extended key usage”, “Key
				Length”, “Version”, “Serial Number”, “Signature Algorithm” and “Validity” data as the
				information of the certificate, as long as the device can retrieve such information from the
				specified certificate.<br/>
				A device that supports either TLS or IEEE 802.1X should support this command.</wsdl:documentation>
			<wsdl:input message="tds:GetCertificateInformationRequest"/>
			<wsdl:output message="tds:GetCertificateInformationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="LoadCACertificates">
			<wsdl:documentation>This command is used when it is necessary to load trusted CA certificates or trusted root
				certificates for the purpose of verification for its counterpart i.e. client certificate verification in
				TLS function or server certificate verification in IEEE 802.1X function.<br/>
				A device that support either TLS or IEEE 802.1X shall support this command. As for the
				supported certificate format, either DER format or PEM format is possible to be used. But a
				device that support this command shall support at least DER format as supported format type.
				The device may sort the received certificate(s) based on the public key and subject
				information in the certificate(s). Either all CA certificates are loaded successfully or a fault
				message shall be returned without loading any CA certificate.</wsdl:documentation>
			<wsdl:input message="tds:LoadCACertificatesRequest"/>
			<wsdl:output message="tds:LoadCACertificatesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateDot1XConfiguration">
			<wsdl:documentation>This operation newly creates IEEE 802.1X configuration parameter set of the device. The
				device shall support this command if it supports IEEE 802.1X. If the device receives this
				request with already existing configuration token (Dot1XConfigurationToken) specification, the
				device should respond with 'ter:ReferenceToken ' error to indicate there is some configuration
				conflict.</wsdl:documentation>
			<wsdl:input message="tds:CreateDot1XConfigurationRequest"/>
			<wsdl:output message="tds:CreateDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDot1XConfiguration">
			<wsdl:documentation>While the CreateDot1XConfiguration command is trying to create a new configuration
				parameter set, this operation modifies existing IEEE 802.1X configuration parameter set of
				the device. A device that support IEEE 802.1X shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:SetDot1XConfigurationRequest"/>
			<wsdl:output message="tds:SetDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfiguration">
			<wsdl:documentation>This operation gets one IEEE 802.1X configuration parameter set from the device by
				specifying the configuration token (Dot1XConfigurationToken).<br/>
				A device that supports IEEE 802.1X shall support this command.
				Regardless of whether the 802.1X method in the retrieved configuration has a password or
				not, the device shall not include the Password element in the response.</wsdl:documentation>
			<wsdl:input message="tds:GetDot1XConfigurationRequest"/>
			<wsdl:output message="tds:GetDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfigurations">
			<wsdl:documentation>This operation gets all the existing IEEE 802.1X configuration parameter sets from the device.
				The device shall respond with all the IEEE 802.1X configurations so that the client can get to
				know how many IEEE 802.1X configurations are existing and how they are configured.<br/>
				A device that support IEEE 802.1X shall support this command.<br/>
				Regardless of whether the 802.1X method in the retrieved configuration has a password or
				not, the device shall not include the Password element in the response.</wsdl:documentation>
			<wsdl:input message="tds:GetDot1XConfigurationsRequest"/>
			<wsdl:output message="tds:GetDot1XConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteDot1XConfiguration">
			<wsdl:documentation>This operation deletes an IEEE 802.1X configuration parameter set from the device. Which
				configuration should be deleted is specified by the 'Dot1XConfigurationToken' in the request.
				A device that support IEEE 802.1X shall support this command.</wsdl:documentation>
			<wsdl:input message="tds:DeleteDot1XConfigurationRequest"/>
			<wsdl:output message="tds:DeleteDot1XConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDot11Capabilities">
			<wsdl:documentation>This operation returns the IEEE802.11 capabilities. The device shall support
				this operation.</wsdl:documentation>
			<wsdl:input message="tds:GetDot11CapabilitiesRequest"/>
			<wsdl:output message="tds:GetDot11CapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDot11Status">
			<wsdl:documentation>This operation returns the status of a wireless network interface. The device shall support this
				command.</wsdl:documentation>
			<wsdl:input message="tds:GetDot11StatusRequest"/>
			<wsdl:output message="tds:GetDot11StatusResponse"/>
		</wsdl:operation>
		<wsdl:operation name="ScanAvailableDot11Networks">
			<wsdl:documentation>This operation returns a lists of the wireless networks in range of the device. A device should
				support this operation.</wsdl:documentation>
			<wsdl:input message="tds:ScanAvailableDot11NetworksRequest"/>
			<wsdl:output message="tds:ScanAvailableDot11NetworksResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSystemUris">
			<wsdl:documentation>This operation is used to retrieve URIs from which system information may be downloaded
				using HTTP. URIs may be returned for the following system information:<br/>
				System Logs. Multiple system logs may be returned, of different types. The exact format of
				the system logs is outside the scope of this specification.<br/>
				Support Information. This consists of arbitrary device diagnostics information from a device.
				The exact format of the diagnostic information is outside the scope of this specification.<br/>
				System Backup. The received file is a backup file that can be used to restore the current
				device configuration at a later date. The exact format of the backup configuration file is
				outside the scope of this specification.<br/>
				If the device allows retrieval of system logs, support information or system backup data, it
				should make them available via HTTP GET. If it does, it shall support the GetSystemUris
				command.</wsdl:documentation>
			<wsdl:input message="tds:GetSystemUrisRequest"/>
			<wsdl:output message="tds:GetSystemUrisResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StartFirmwareUpgrade">
			<wsdl:documentation>This operation initiates a firmware upgrade using the HTTP POST mechanism. The response
				to the command includes an HTTP URL to which the upgrade file may be uploaded. The
				actual upgrade takes place as soon as the HTTP POST operation has completed. The device
				should support firmware upgrade through the StartFirmwareUpgrade command. The exact
				format of the firmware data is outside the scope of this specification.
				Firmware upgrade over HTTP may be achieved using the following steps:<ol>
					<li>Client calls StartFirmwareUpgrade.</li>
					<li>Server responds with upload URI and optional delay value.</li>
					<li>Client waits for delay duration if specified by server.</li>
					<li>Client transmits the firmware image to the upload URI using HTTP POST.</li>
					<li>Server reprograms itself using the uploaded image, then reboots.</li>
				</ol>
				If the firmware upgrade fails because the upgrade file was invalid, the HTTP POST response
				shall be “415 Unsupported Media Type”. If the firmware upgrade fails due to an error at the
				device, the HTTP POST response shall be “500 Internal Server Error”.<br/>
				The value of the Content-Type header in the HTTP POST request shall be “application/octetstream”.</wsdl:documentation>
			<wsdl:input message="tds:StartFirmwareUpgradeRequest"/>
			<wsdl:output message="tds:StartFirmwareUpgradeResponse"/>
		</wsdl:operation>
		<wsdl:operation name="StartSystemRestore">
			<wsdl:documentation>This operation initiates a system restore from backed up configuration data using the HTTP
				POST mechanism. The response to the command includes an HTTP URL to which the backup
				file may be uploaded. The actual restore takes place as soon as the HTTP POST operation
				has completed. Devices should support system restore through the StartSystemRestore
				command. The exact format of the backup configuration data is outside the scope of this
				specification.<br/>
				System restore over HTTP may be achieved using the following steps:<ol>
					<li>Client calls StartSystemRestore.</li>
					<li>Server responds with upload URI.</li>
					<li>Client transmits the configuration data to the upload URI using HTTP POST.</li>
					<li>Server applies the uploaded configuration, then reboots if necessary.</li>
				</ol>
				If the system restore fails because the uploaded file was invalid, the HTTP POST response
				shall be “415 Unsupported Media Type”. If the system restore fails due to an error at the
				device, the HTTP POST response shall be “500 Internal Server Error”.<br/>
				The value of the Content-Type header in the HTTP POST request shall be “application/octetstream”.</wsdl:documentation>
			<wsdl:input message="tds:StartSystemRestoreRequest"/>
			<wsdl:output message="tds:StartSystemRestoreResponse"/>
		</wsdl:operation>

		<wsdl:operation name="GetStorageConfigurations">
			<wsdl:documentation>
			This operation lists all existing storage configurations for the device. 
			</wsdl:documentation>
			<wsdl:input message="tds:GetStorageConfigurationsRequest"/>
			<wsdl:output message="tds:GetStorageConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="CreateStorageConfiguration">
			<wsdl:documentation>
			This operation creates a new storage configuration. 
			The configuration data shall be created in the device and shall be persistent (remain after reboot).
			</wsdl:documentation>
			<wsdl:input message="tds:CreateStorageConfigurationRequest"/>
			<wsdl:output message="tds:CreateStorageConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetStorageConfiguration">
			<wsdl:documentation>
			This operation retrieves the Storage configuration associated with the given storage configuration token.
			</wsdl:documentation>
			<wsdl:input message="tds:GetStorageConfigurationRequest"/>
			<wsdl:output message="tds:GetStorageConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetStorageConfiguration">
			<wsdl:documentation>
			This operation modifies an existing Storage configuration.
			</wsdl:documentation>
			<wsdl:input message="tds:SetStorageConfigurationRequest"/>
			<wsdl:output message="tds:SetStorageConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteStorageConfiguration">
			<wsdl:documentation>
			This operation deletes the given storage configuration and configuration change shall always be persistent.
			</wsdl:documentation>
			<wsdl:input message="tds:DeleteStorageConfigurationRequest"/>
			<wsdl:output message="tds:DeleteStorageConfigurationResponse"/>
		</wsdl:operation>

		<wsdl:operation name="GetGeoLocation">
			<wsdl:documentation>
				This operation lists all existing geo location configurations for the device. 
			</wsdl:documentation>
			<wsdl:input message="tds:GetGeoLocationRequest"/>
			<wsdl:output message="tds:GetGeoLocationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetGeoLocation">
			<wsdl:documentation>
				This operation allows to modify one or more geo configuration entries.
			</wsdl:documentation>
			<wsdl:input message="tds:SetGeoLocationRequest"/>
			<wsdl:output message="tds:SetGeoLocationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<wsdl:documentation>
				This operation deletes the given geo location entries.
			</wsdl:documentation>
			<wsdl:input message="tds:DeleteGeoLocationRequest"/>
			<wsdl:output message="tds:DeleteGeoLocationResponse"/>
		</wsdl:operation>
		
	</wsdl:portType>
	<wsdl:binding name="DeviceBinding" type="tds:Device">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServices">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetServices"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDeviceInformation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDeviceInformation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetSystemDateAndTime">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetSystemDateAndTime"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSystemDateAndTime">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetSystemDateAndTime"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetSystemFactoryDefault">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetSystemFactoryDefault"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="UpgradeSystemFirmware">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/UpgradeSystemFirmware"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SystemReboot">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SystemReboot"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RestoreSystem">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/RestoreSystem"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSystemBackup">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetSystemBackup"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSystemLog">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetSystemLog"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSystemSupportInformation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetSystemSupportInformation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetScopes">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetScopes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetScopes">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetScopes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="AddScopes">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/AddScopes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemoveScopes">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/RemoveScopes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDiscoveryMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDiscoveryMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDiscoveryMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetDiscoveryMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRemoteDiscoveryMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetRemoteDiscoveryMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRemoteDiscoveryMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetRemoteDiscoveryMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDPAddresses">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDPAddresses"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetEndpointReference">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetEndpointReference"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRemoteUser">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetRemoteUser"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRemoteUser">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetRemoteUser"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetUsers">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetUsers"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateUsers">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/CreateUsers"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteUsers">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteUsers"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetUser">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetUser"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetWsdlUrl">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetWsdlUrl"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDPAddresses">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetDPAddresses"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetHostname">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetHostname"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetHostname">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetHostname"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetHostnameFromDHCP">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetHostnameFromDHCP"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDNS">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDNS"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDNS">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetDNS"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNTP">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetNTP"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetNTP">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetNTP"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDynamicDNS">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDynamicDNS"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDynamicDNS">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetDynamicDNS"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkInterfaces">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetNetworkInterfaces"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkInterfaces">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetNetworkInterfaces"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkProtocols">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetNetworkProtocols"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkProtocols">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetNetworkProtocols"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetNetworkDefaultGateway">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetNetworkDefaultGateway"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetNetworkDefaultGateway">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetNetworkDefaultGateway"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetZeroConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetZeroConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetZeroConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetZeroConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetIPAddressFilter">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetIPAddressFilter"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetIPAddressFilter">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetIPAddressFilter"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="AddIPAddressFilter">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/AddIPAddressFilter"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="RemoveIPAddressFilter">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/RemoveIPAddressFilter"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAccessPolicy">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetAccessPolicy"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAccessPolicy">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetAccessPolicy"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateCertificate">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/CreateCertificate"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetCertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertificatesStatus">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetCertificatesStatus"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetCertificatesStatus">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetCertificatesStatus"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteCertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteCertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetPkcs10Request">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetPkcs10Request"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="LoadCertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/LoadCertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetClientCertificateMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetClientCertificateMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetClientCertificateMode">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetClientCertificateMode"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetRelayOutputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputSettings">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetRelayOutputSettings"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputState">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetRelayOutputState"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SendAuxiliaryCommand">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SendAuxiliaryCommand"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCACertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetCACertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="LoadCertificateWithPrivateKey">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/LoadCertificateWithPrivateKey"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetCertificateInformation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetCertificateInformation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="LoadCACertificates">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/LoadCACertificates"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/CreateDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDot1XConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDot1XConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteDot1XConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteDot1XConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDot11Capabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDot11Capabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDot11Status">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetDot11Status"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="ScanAvailableDot11Networks">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/ScanAvailableDot11Networks"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSystemUris">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetSystemUris"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="StartFirmwareUpgrade">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/StartFirmwareUpgrade"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="StartSystemRestore">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/StartSystemRestore"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>

		<wsdl:operation name="GetStorageConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetStorageConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="CreateStorageConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/CreateStorageConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetStorageConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetStorageConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetStorageConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetStorageConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteStorageConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteStorageConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>

		<wsdl:operation name="GetGeoLocation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetGeoLocation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetGeoLocation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/SetGeoLocation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteGeoLocation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		
		
	</wsdl:binding>
	<!--===============================-->
	<!--===============================-->
</wsdl:definitions>
