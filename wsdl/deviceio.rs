<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2017 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" xmlns:tmd="http://www.onvif.org/ver10/deviceIO/wsdl" targetNamespace="http://www.onvif.org/ver10/deviceIO/wsdl">
	<wsdl:import namespace="http://www.onvif.org/ver10/device/wsdl" location="../ver10/device/wsdl/devicemgmt.wsdl"/>
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver10/deviceIO/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="17.12">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../ver10/schema/onvif.xsd"/>
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tmd:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the device IO service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="VideoSources" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of video sources (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="VideoOutputs" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of video outputs (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="AudioSources" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of audio sources (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="AudioOutputs" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of audio outputs (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="RelayOutputs" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of relay outputs (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="SerialPorts" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of serial ports (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DigitalInputs" type="xs:int" default="0">
					<xs:annotation>
						<xs:documentation>Number of digital inputs (defaults to none).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:attribute name="DigitalInputOptions" type="xs:boolean" default="false">
					<xs:annotation>
						<xs:documentation>Indicates support for DigitalInput configuration of the idle state (defaults to false).</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="tmd:Capabilities"/>
			<!--===============================-->
			<xs:element name="GetRelayOutputOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutputToken" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>
								Optional reference token to the relay for which the options are requested.
							</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRelayOutputOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutputOptions" type="tmd:RelayOutputOptions" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>
								Valid values and ranges for the configuration of a relay output.
							</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:complexType name="RelayOutputOptions">
				<xs:sequence>
					<xs:element name="Mode" type="tt:RelayMode" maxOccurs="unbounded">
						<xs:annotation>
							<xs:documentation>Supported Modes.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="DelayTimes" type="tmd:DelayTimes" minOccurs="0">
						<xs:annotation>
							<xs:documentation>Supported delay time range or discrete values in seconds. This element must be present if MonoStable mode is supported.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Discrete" type="xs:boolean" minOccurs="0">
						<xs:annotation>
							<xs:documentation>True if the relay only supports the exact values for the DelayTimes listed. Default is false.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="Extension" type="tmd:RelayOutputOptionsExtension" minOccurs="0"/>
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken" use="required">
					<xs:annotation>
						<xs:documentation>Token of the relay output.</xs:documentation>
					</xs:annotation>
				</xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:complexType name="RelayOutputOptionsExtension">
				<xs:sequence>
					<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
			</xs:complexType>
			<xs:simpleType name="DelayTimes">
				<xs:list itemType="xs:float"/>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="Get">
				<xs:sequence>
				</xs:sequence>
			</xs:complexType>
			<xs:complexType name="GetResponse">
				<xs:sequence>
					<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0" maxOccurs="unbounded">
						<xs:annotation>
							<xs:documentation>List tokens of a physical IO of a device.</xs:documentation>
						</xs:annotation>
					</xs:element>
				</xs:sequence>
			</xs:complexType>
			<xs:element name="GetVideoSources" type="tmd:Get"/>
			<xs:element name="GetVideoSourcesResponse" type="tmd:GetResponse"/>
			<xs:element name="GetAudioSources" type="tmd:Get"/>
			<xs:element name="GetAudioSourcesResponse" type="tmd:GetResponse"/>
			<xs:element name="GetAudioOutputs" type="tmd:Get"/>
			<xs:element name="GetAudioOutputsResponse" type="tmd:GetResponse"/>
			<!--===============================-->
			<xs:element name="GetVideoOutputs">
				<xs:complexType>
					<xs:sequence>
				   </xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoOutputsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoOutputs" type="tt:VideoOutput" minOccurs="0" maxOccurs="unbounded">
							<xs:annotation>
								<xs:documentation>List containing all physical Video output connections of a device.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested AudioSource.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioSourceConfiguration" type="tt:AudioSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Current configuration of the Audio input.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioOutputToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the physical Audio output.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioOutputConfiguration" type="tt:AudioOutputConfiguration">
							<xs:annotation>
								<xs:documentation>Current configuration of the Audio output.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested VideoSource.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceConfiguration" type="tt:VideoSourceConfiguration">
							<xs:annotation>
								<xs:documentation>Current configuration of the Video input.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoOutputToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the requested VideoOutput.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoOutputConfiguration" type="tt:VideoOutputConfiguration">
							<xs:annotation>
								<xs:documentation>Current configuration of the Video output.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAudioSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioSourceConfiguration"/>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element determines how configuration
							changes shall be stored. If true, changes shall be persistent. If false, changes MAY revert to previous values
							after reboot.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetAudioOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:AudioOutputConfiguration"/>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element determines how configuration
							changes shall be stored. If true, changes shall be persistent. If false, changes MAY revert to previous values
							after reboot.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetAudioOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetVideoSourceConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoSourceConfiguration"/>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element determines how configuration
							changes shall be stored. If true, changes shall be persistent. If false, changes MAY revert to previous values
							after reboot.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoSourceConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetVideoOutputConfiguration">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Configuration" type="tt:VideoOutputConfiguration"/>
						<xs:element name="ForcePersistence" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>The ForcePersistence element determines how configuration
							changes shall be stored. If true, changes shall be persistent. If false, changes MAY revert to previous values
							after reboot.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetVideoOutputConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoSourceConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the Video input whose options are requested..</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoSourceConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoSourceConfigurationOptions" type="tt:VideoSourceConfigurationOptions"/>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetVideoOutputConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoOutputToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the Video Output whose options are requested..</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetVideoOutputConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="VideoOutputConfigurationOptions" type="tt:VideoOutputConfigurationOptions"/>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioSourceConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioSourceToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the physical Audio input whose options are requested..</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioSourceConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioSourceOptions" type="tt:AudioSourceConfigurationOptions">
							<xs:annotation>
								<xs:documentation>Returns the AudioSourceToken available.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetAudioOutputConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioOutputToken" type="tt:ReferenceToken">
							<xs:annotation>
								<xs:documentation>Token of the physical Audio Output whose options are requested..</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetAudioOutputConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="AudioOutputOptions" type="tt:AudioOutputConfigurationOptions">
							<xs:annotation>
								<xs:documentation>Available settings and ranges for the requested Audio output.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:any namespace="##any" minOccurs="0" maxOccurs="unbounded" processContents="lax"/>   <!-- first Vendor then ONVIF -->
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetRelayOutputSettings">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RelayOutput" type="tt:RelayOutput"/>
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
			<xs:element name="GetDigitalInputs">
				<xs:annotation>
					<xs:documentation>Get the available digital inputs of a device.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDigitalInputsResponse">
				<xs:annotation>
					<xs:documentation>Requested digital inputs.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DigitalInputs" type="tt:DigitalInput" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="DigitalInputConfigurationOptions">
				<xs:sequence>
					<xs:element name="IdleState" type="tt:DigitalIdleState" maxOccurs="unbounded">
						<xs:annotation>
							<xs:documentation>Configuration Options for a digital input.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first ONVIF then Vendor -->
				</xs:sequence>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:element name="GetDigitalInputConfigurationOptions">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetDigitalInputConfigurationOptionsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DigitalInputOptions" type="tmd:DigitalInputConfigurationOptions"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetDigitalInputConfigurations">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="DigitalInputs" type="tt:DigitalInput" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetDigitalInputConfigurationsResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSerialPorts">
				<xs:annotation>
					<xs:documentation>The physical serial port on the device that allows serial data to be read and written.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSerialPortsResponse">
				<xs:annotation>
					<xs:documentation>Requested serial ports.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPort" type="tmd:SerialPort" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSerialPortConfiguration">
				<xs:annotation>
					<xs:documentation>Gets the configuration that relates to serial port configuration.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPortToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSerialPortConfigurationResponse">
				<xs:annotation>
					<xs:documentation>Requested serial port configuration.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPortConfiguration" type="tmd:SerialPortConfiguration"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SetSerialPortConfiguration">
				<xs:annotation>
					<xs:documentation>Sets the configuration that relates to serial port configuration.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPortConfiguration" type="tmd:SerialPortConfiguration"/>
						<xs:element name="ForcePersistance" type="xs:boolean"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SetSerialPortConfigurationResponse">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSerialPortConfigurationOptions">
				<xs:annotation>
					<xs:documentation>Gets the configuration options that relates to serial port configuration.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPortToken" type="tt:ReferenceToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSerialPortConfigurationOptionsResponse">
				<xs:annotation>
					<xs:documentation>Requested serial port configuration options.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialPortOptions" type="tmd:SerialPortConfigurationOptions"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="SendReceiveSerialCommand">
				<xs:annotation>
					<xs:documentation>Transmitting arbitrary data to the connected serial device and then receiving its response data.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Token" type="tt:ReferenceToken" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The physical serial port reference to be used when this request is invoked.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="SerialData" type="tmd:SerialData" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The serial port data.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="TimeOut" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>Indicates that the command should be responded back within the specified period of time.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="DataLength" type="xs:integer" minOccurs="0">
							<xs:annotation>
								<xs:documentation>This element may be put in the case that data length returned from the connected serial device is already determined as some fixed bytes length. It indicates the length of received data which can be regarded as available.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Delimiter" type="xs:string" minOccurs="0">
							<xs:annotation>
								<xs:documentation>This element may be put in the case that the delimiter codes returned from the connected serial device is already known. It indicates the termination data sequence of the responded data. In case the string has more than one character a device shall interpret the whole string as a single delimiter. Furthermore a device shall return the delimiter character(s) to the client.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="SendReceiveSerialCommandResponse">
				<xs:annotation>
					<xs:documentation>Receiving the response data.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SerialData" type="tmd:SerialData" minOccurs="0"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--==                Serial port types            ===-->
			<!--===============================-->
			<xs:complexType name="SerialData">
				<xs:annotation>
					<xs:documentation>The serial port data.</xs:documentation>
				</xs:annotation>
				<xs:choice>
					<xs:element name="Binary" type="xs:base64Binary"/>
					<xs:element name="String" type="xs:string"/>
				</xs:choice>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="SerialPort">
				<xs:annotation>
					<xs:documentation>Lists all available serial ports of a device</xs:documentation>
				</xs:annotation>
				<xs:complexContent>
					<xs:extension base="tt:DeviceEntity">
						<xs:sequence>
							<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
						</xs:sequence>
						<xs:anyAttribute processContents="lax"/>
					</xs:extension>
				</xs:complexContent>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="SerialPortType">
				<xs:annotation>
					<xs:documentation>The type of serial port.Generic can be signaled as a vendor specific serial port type.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:string">
					<xs:enumeration value="RS232"/>
					<xs:enumeration value="RS422HalfDuplex"/>
					<xs:enumeration value="RS422FullDuplex"/>
					<xs:enumeration value="RS485HalfDuplex"/>
					<xs:enumeration value="RS485FullDuplex"/>
					<xs:enumeration value="Generic"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="SerialPortConfiguration">
				<xs:annotation>
					<xs:documentation>The parameters for configuring the serial port.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="BaudRate" type="xs:int">
						<xs:annotation>
							<xs:documentation>The transfer bitrate.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="ParityBit" type="tmd:ParityBit">
						<xs:annotation>
							<xs:documentation>The parity for the data error detection.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="CharacterLength" type="xs:int">
						<xs:annotation>
							<xs:documentation>The bit length for each character.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="StopBit" type="xs:float">
						<xs:annotation>
							<xs:documentation>The number of stop bits used to terminate each character.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken" use="required"/>
				<xs:attribute name="type" type="tmd:SerialPortType" use="required"/>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:simpleType name="ParityBit">
				<xs:annotation>
					<xs:documentation>The parity for the data error detection.</xs:documentation>
				</xs:annotation>
				<xs:restriction base="xs:string">
					<xs:enumeration value="None"/>
					<xs:enumeration value="Even"/>
					<xs:enumeration value="Odd"/>
					<xs:enumeration value="Mark"/>
					<xs:enumeration value="Space"/>
					<xs:enumeration value="Extended"/>
				</xs:restriction>
			</xs:simpleType>
			<!--===============================-->
			<xs:complexType name="SerialPortConfigurationOptions">
				<xs:annotation>
					<xs:documentation>The configuration options that relates to serial port.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="BaudRateList" type="tt:IntList">
						<xs:annotation>
							<xs:documentation>The list of configurable transfer bitrate.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="ParityBitList" type="tmd:ParityBitList">
						<xs:annotation>
							<xs:documentation>The list of configurable parity for the data error detection.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="CharacterLengthList" type="tt:IntList">
						<xs:annotation>
							<xs:documentation>The list of configurable bit length for each character.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:element name="StopBitList" type="tt:FloatList">
						<xs:annotation>
							<xs:documentation>The list of configurable number of stop bits used to terminate each character.</xs:documentation>
						</xs:annotation>
					</xs:element>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>   <!-- first Vendor then ONVIF -->
				</xs:sequence>
				<xs:attribute name="token" type="tt:ReferenceToken" use="required"/>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<!--===============================-->
			<xs:complexType name="ParityBitList">
				<xs:annotation>
					<xs:documentation>The list of configurable parity for the data error detection.</xs:documentation>
				</xs:annotation>
				<xs:sequence>
					<xs:element name="Items" type="tmd:ParityBit" minOccurs="0" maxOccurs="unbounded"/>
				</xs:sequence>
			</xs:complexType>
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tmd:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tmd:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetRelayOutputOptions"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetRelayOutputOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputsRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputs"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputsResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputsRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputs"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputsResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourcesRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoSources"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourcesResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoSourcesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourcesRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioSources"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourcesResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioSourcesResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:SetVideoSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:SetVideoSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetVideoOutputConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:SetVideoOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetVideoOutputConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:SetVideoOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:SetAudioSourceConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioSourceConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:SetAudioSourceConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:SetAudioOutputConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetAudioOutputConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:SetAudioOutputConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetVideoOutputConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetVideoOutputConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioSourceConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioSourceConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioSourceConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetAudioOutputConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetAudioOutputConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputsRequest">
		<wsdl:part name="parameters" element="tds:GetRelayOutputs"/>
	</wsdl:message>
	<wsdl:message name="GetRelayOutputsResponse">
		<wsdl:part name="parameters" element="tds:GetRelayOutputsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputSettingsRequest">
		<wsdl:part name="parameters" element="tmd:SetRelayOutputSettings"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputSettingsResponse">
		<wsdl:part name="parameters" element="tmd:SetRelayOutputSettingsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputStateRequest">
		<wsdl:part name="parameters" element="tds:SetRelayOutputState"/>
	</wsdl:message>
	<wsdl:message name="SetRelayOutputStateResponse">
		<wsdl:part name="parameters" element="tds:SetRelayOutputStateResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDigitalInputsRequest">
		<wsdl:part name="parameters" element="tmd:GetDigitalInputs"/>
	</wsdl:message>
	<wsdl:message name="GetDigitalInputsResponse">
		<wsdl:part name="parameters" element="tmd:GetDigitalInputsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetDigitalInputConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetDigitalInputConfigurationOptions"/>
	</wsdl:message>                                         
	<wsdl:message name="GetDigitalInputConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetDigitalInputConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="SetDigitalInputConfigurationsRequest">
		<wsdl:part name="parameters" element="tmd:SetDigitalInputConfigurations"/>
	</wsdl:message>
	<wsdl:message name="SetDigitalInputConfigurationsResponse">
		<wsdl:part name="parameters" element="tmd:SetDigitalInputConfigurationsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortsRequest">
		<wsdl:part name="parameters" element="tmd:GetSerialPorts"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortsResponse">
		<wsdl:part name="parameters" element="tmd:GetSerialPortsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:GetSerialPortConfiguration"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:GetSerialPortConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="SetSerialPortConfigurationRequest">
		<wsdl:part name="parameters" element="tmd:SetSerialPortConfiguration"/>
	</wsdl:message>
	<wsdl:message name="SetSerialPortConfigurationResponse">
		<wsdl:part name="parameters" element="tmd:SetSerialPortConfigurationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortConfigurationOptionsRequest">
		<wsdl:part name="parameters" element="tmd:GetSerialPortConfigurationOptions"/>
	</wsdl:message>
	<wsdl:message name="GetSerialPortConfigurationOptionsResponse">
		<wsdl:part name="parameters" element="tmd:GetSerialPortConfigurationOptionsResponse"/>
	</wsdl:message>
	<wsdl:message name="SendReceiveSerialCommandRequest">
		<wsdl:part name="parameters" element="tmd:SendReceiveSerialCommand"/>
	</wsdl:message>
	<wsdl:message name="SendReceiveSerialCommandResponse">
		<wsdl:part name="parameters" element="tmd:SendReceiveSerialCommandResponse"/>
	</wsdl:message>
	<wsdl:portType name="DeviceIOPort">
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the device IO service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tmd:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tmd:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputOptions">
			<wsdl:documentation>Request the available settings and ranges for one or all relay outputs. A device that has one or more RelayOutputs should support this command.<br/>
				Two examples that illustrate usage:
				<ul>
					<li>
					1) Device supports range PT1S to PT120S:
					<pre>
&lt;tmd:RelayOutputOptions token=&apos;44&apos;&gt;
  &lt;tmd:Mode&gt;Monostable&lt;/tmd:Mode&gt;
  &lt;tmd:DelayTimes&gt;1 120&lt;/tmd:DelayTimes&gt;
&lt;/tmd:RelayOutputOptions&gt;
							</pre>
					</li>
					<li>
					2) Device supports values PT0.5S, PT1S, PT2s and PT1M:
					<pre>
&lt;tmd:RelayOutputOptions token=&apos;123&apos;&gt;
  &lt;tmd:Mode&gt;Monostable&lt;/tmd:Mode&gt;
  &lt;tmd:DelayTimes&gt;0.5 1 2 60&lt;/tmd:DelayTimes&gt;
  &lt;tmd:Discrete&gt;True&lt;/tmd:Discrete&gt;
&lt;/tmd:RelayOutputOptions&gt;
								</pre>
					</li>
				</ul>
			</wsdl:documentation>
			<wsdl:input message="tmd:GetRelayOutputOptionsRequest"/>
			<wsdl:output message="tmd:GetRelayOutputOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSources">
			<wsdl:documentation>List all available audio sources for the device. The device that has one or more audio sources shall support the listing of available audio inputs through the GetAudioSources command.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioSourcesRequest"/>
			<wsdl:output message="tmd:GetAudioSourcesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputs">
			<wsdl:documentation>List all available audio outputs of a device. A device that has one ore more physical audio outputs shall support listing of available audio outputs through the GetAudioOutputs command.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioOutputsRequest"/>
			<wsdl:output message="tmd:GetAudioOutputsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoSources">
			<wsdl:documentation>List all available video sources for the device. The device that has one or more video inputs shall support the listing of available video sources through the GetVideoSources command.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoSourcesRequest"/>
			<wsdl:output message="tmd:GetVideoSourcesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputs">
			<wsdl:documentation>List all available video outputs of a device. A device that has one or more physical video outputs shall support listing of available video outputs through the GetVideoOutputs command.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoOutputsRequest"/>
			<wsdl:output message="tmd:GetVideoOutputsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfiguration">
			<wsdl:documentation>Get the video source configurations of a VideoSource. A device with one or more video sources shall support the GetVideoSourceConfigurations command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoSourceConfigurationRequest"/>
			<wsdl:output message="tmd:GetVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputConfiguration">
			<wsdl:documentation>Get the configuration of a Video Output. A device that has one or more Video Outputs shall support the retrieval of the VideoOutputConfiguration through this command.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoOutputConfigurationRequest"/>
			<wsdl:output message="tmd:GetVideoOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfiguration">
			<wsdl:documentation>List the configuration of an Audio Input. A device with one or more audio inputs shall support the GetAudioSourceConfiguration command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioSourceConfigurationRequest"/>
			<wsdl:output message="tmd:GetAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfiguration">
			<wsdl:documentation>Request the current configuration of a physical Audio output. A device that has one or more AudioOutputs shall support the retrieval of the AudioOutputConfiguration through this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioOutputConfigurationRequest"/>
			<wsdl:output message="tmd:GetAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<wsdl:documentation>Modify a video input configuration. A device that has one or more video sources shall support the setting of the VideoSourceConfiguration through this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:SetVideoSourceConfigurationRequest"/>
			<wsdl:output message="tmd:SetVideoSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetVideoOutputConfiguration">
			<wsdl:documentation>Modify a video output configuration. A device that has one or more video outputs shall support the setting of its video output configuration through this command.</wsdl:documentation>
			<wsdl:input message="tmd:SetVideoOutputConfigurationRequest"/>
			<wsdl:output message="tmd:SetVideoOutputConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<wsdl:documentation>Modify an audio source configuration. A device that has a one or more audio sources shall support the setting of the AudioSourceConfiguration through this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:SetAudioSourceConfigurationRequest"/>
			<wsdl:output message="tmd:SetAudioSourceConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<wsdl:documentation>Modify an audio output configuration. A device that has one ore more audio outputs shall support the setting of the AudioOutputConfiguration through this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:SetAudioOutputConfigurationRequest"/>
			<wsdl:output message="tmd:SetAudioOutputConfigurationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<wsdl:documentation>Request the VideoSourceConfigurationOptions of a VideoSource. A device with one or more video sources shall support this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoSourceConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetVideoSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputConfigurationOptions">
			<wsdl:documentation>Request the VideoOutputConfigurationOptions of a VideoOutput. A device that has one or more video outputs shall support the retrieval of VideoOutputConfigurationOptions through this command.</wsdl:documentation>
			<wsdl:input message="tmd:GetVideoOutputConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetVideoOutputConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<wsdl:documentation>Request the AudioSourceConfigurationOptions of an AudioSource. A device with one ore more AudioSources shall support this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioSourceConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetAudioSourceConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<wsdl:documentation>Request the available settings and ranges for a physical Audio output. A device that has one or more AudioOutputs shall support this command.<br/>This method is deprecated.</wsdl:documentation>
			<wsdl:input message="tmd:GetAudioOutputConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetAudioOutputConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputs">
			<wsdl:documentation>This operation gets a list of all available relay outputs and their settings.</wsdl:documentation>
			<wsdl:input message="tmd:GetRelayOutputsRequest"/>
			<wsdl:output message="tmd:GetRelayOutputsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputSettings">
			<wsdl:documentation>This operation sets the settings of a relay output.
				The relay can work in two relay modes: <ul>
					<li>
				Bistable – After setting the state, the relay remains in this state.</li>
					<li>
				Monostable – After setting the state, the relay returns to its idle state after the
				specified time.</li>
				</ul>
				The physical idle state of a relay output can be configured by setting the IdleState to ‘open’ or
				‘closed’ (inversion of the relay behaviour).<br/>
				Idle State ‘open’ means that the relay is open when the relay state is set to ‘inactive’ through
				the trigger command (see Section 8.5.3) and closed when the state is set to ‘active’ through
				the same command.<br/>
				Idle State ‘closed’ means, that the relay is closed when the relay state is set to ‘inactive’
				through the trigger command (see Section 8.5.3) and open when the state is set to ‘active’
				through the same command.</wsdl:documentation>
			<wsdl:input message="tmd:SetRelayOutputSettingsRequest"/>
			<wsdl:output message="tmd:SetRelayOutputSettingsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputState">
			<wsdl:documentation>Modify the relay state.</wsdl:documentation>
			<wsdl:input message="tmd:SetRelayOutputStateRequest"/>
			<wsdl:output message="tmd:SetRelayOutputStateResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDigitalInputs">
			<wsdl:documentation>This operation gets a list of all available digital inputs.</wsdl:documentation>
			<wsdl:input message="tmd:GetDigitalInputsRequest"/>
			<wsdl:output message="tmd:GetDigitalInputsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetDigitalInputConfigurationOptions">
			<wsdl:documentation>This operation lists what configuration is available for digital inputs.</wsdl:documentation>
			<wsdl:input message="tmd:GetDigitalInputConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetDigitalInputConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetDigitalInputConfigurations">
			<wsdl:documentation>Modify a digital input configuration.</wsdl:documentation>
			<wsdl:input message="tmd:SetDigitalInputConfigurationsRequest"/>
			<wsdl:output message="tmd:SetDigitalInputConfigurationsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPorts">
			<wsdl:input message="tmd:GetSerialPortsRequest"/>
			<wsdl:output message="tmd:GetSerialPortsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPortConfiguration">
			<wsdl:input message="tmd:GetSerialPortConfigurationRequest"/>
			<wsdl:output message="tmd:GetSerialPortConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SetSerialPortConfiguration">
			<wsdl:input message="tmd:SetSerialPortConfigurationRequest"/>
			<wsdl:output message="tmd:SetSerialPortConfigurationResponse"/>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPortConfigurationOptions">
			<wsdl:input message="tmd:GetSerialPortConfigurationOptionsRequest"/>
			<wsdl:output message="tmd:GetSerialPortConfigurationOptionsResponse"/>
		</wsdl:operation>
		<wsdl:operation name="SendReceiveSerialCommand">
			<wsdl:input message="tmd:SendReceiveSerialCommandRequest"/>
			<wsdl:output message="tmd:SendReceiveSerialCommandResponse"/>
		</wsdl:operation>
	</wsdl:portType>
	<wsdl:binding name="DeviceIOBinding" type="tmd:DeviceIOPort">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetRelayOutputOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSources">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioSources"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioOutputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoSources">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoSources"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoOutputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoOutputConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="SetVideoSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetVideoSourceConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetVideoOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetVideoOutputConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioSourceConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetAudioSourceConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetAudioOutputConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetAudioOutputConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetVideoSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoSourceConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetVideoOutputConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetVideoOutputConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioSourceConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioSourceConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetAudioOutputConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetAudioOutputConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetRelayOutputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetRelayOutputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputSettings">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetRelayOutputSettings"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetRelayOutputState">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetRelayOutputState"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDigitalInputs">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetDigitalInputs"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetDigitalInputConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetDigitalInputConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetDigitalInputConfigurations">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetDigitalInputConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPorts">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetSerialPorts"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPortConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetSerialPortConfigurations"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SetSerialPortConfiguration">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SetSerialPortConfiguration"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="GetSerialPortConfigurationOptions">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/GetSerialPortConfigurationOptions"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="SendReceiveSerialCommand">
			<soap:operation soapAction="http://www.onvif.org/ver10/deviceio/wsdl/SendReceiveSerialCommand"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
</wsdl:definitions>
